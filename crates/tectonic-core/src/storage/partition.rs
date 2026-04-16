// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::BinaryHeap;
use rayon::prelude::*;

use crate::error::TectonicError;
use crate::metrics::cluster_metrics::ClusterMetrics;
use crate::quantization::quantized_entry::QuantizedEntry;
use crate::result::{MergeResult, SearchResult};
use crate::search::distance::{SearchMethod};
use crate::utility::typings::{DimVector};
use crate::storage::shard::{CacheShard};
use crate::storage::location::{ShardEntry};
use crate::utility::utils::{calculate_sizes, hash_shard_entry, secondary_arena_hash};

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    pub partition_id: u32,
    pub centroid: Option<[f32; D]>,
    pub size: u64,
    pub capacity: u64,
    pub shards: Vec<CacheShard<D>>,

    // Internal Partition-metrics.
    pub metrics: ClusterMetrics
}

#[allow(dead_code)]
impl<const D: usize> CachePartition<D> {
    pub fn with_capacity(partition_id: u32, capacity: u64, num_shards: u32) -> Self {
        let shard_sizes = calculate_sizes(capacity as usize, num_shards as usize);

        let mut shard_vectors = Vec::with_capacity(shard_sizes.len());
        for (id, &cap) in shard_sizes.iter().enumerate() {
            shard_vectors.push(CacheShard::with_capacity(id as u32, cap));
        }

        Self { 
            partition_id,
            centroid: None,
            size: 0, 
            capacity, 
            shards: shard_vectors,
            metrics: ClusterMetrics::default(),
        }
    }

    pub fn search<M>(
        &self,
        vector: &QuantizedEntry,
        search_method: &M,
        k: usize
    ) -> Result<Vec<SearchResult>, TectonicError>
    where M: SearchMethod<D> + Sync {
        if k == 0 || self.shards.is_empty() {
            return Ok(Vec::new());
        }

        let shard_results: Result<Vec<Vec<SearchResult>>, TectonicError> = self
            .shards
            .par_iter()
            .map(| shard | shard.search(vector, search_method, k))
            .collect();

        let shard_results = shard_results?;
        let results = Self::merge_search_results(shard_results, k);
        Ok(results)
    }

    fn merge_search_results(results: Vec<Vec<SearchResult>>, k: usize) -> Vec<SearchResult> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap: BinaryHeap<MergeResult> = BinaryHeap::with_capacity(results.len());

        for (index, result) in results.iter().enumerate() {
            if let Some(first) = result.first() {
                heap.push(
                    MergeResult {
                        result: *first,
                        shard_index: index,
                        result_index: 0,
                    }
                );
            } 
        }

        let mut output = Vec::with_capacity(k);

        while output.len() < k {
            let item = match heap.pop() {
                Some(item) => item,
                None => break,
            };

            output.push(item.result);

            let next_index = item.result_index + 1;
            if let Some(next_result) = results[item.shard_index].get(next_index) {
                heap.push(
                    MergeResult { 
                        result: *next_result,
                        shard_index: item.shard_index,
                        result_index: next_index 
                    }
                );
            }
        }

        output
    }

    #[inline]
    pub fn increase_centroid_average(&mut self, vector: &DimVector<D>) -> Result<bool, TectonicError> {
        let centroid = self.centroid.as_mut().ok_or_else(|| {
            TectonicError::CentroidError { message: "No centroid available!" }
        })?;

        let old_n = self.size;
        let new_n = old_n + 1;
        let inv_new_avg = 1.0f32 / (new_n as f32);

        for index in 0..D {
            centroid[index] = (centroid[index] * old_n as f32 + vector[index]) * inv_new_avg;
        }

        self.size = new_n;
        Ok(true)
    }

    #[inline]
    pub fn decrease_centroid_average(&mut self, vector: &DimVector<D>) -> Result<bool, TectonicError> {
        let centroid = self.centroid.as_mut().ok_or_else(|| {
            TectonicError::CentroidError { message: "No centroid available!" }
        })?;

        let old_n = self.size;
        let new_n = old_n - 1;
        let inv_new_avg = 1.0f32 / (new_n as f32);

        if old_n <= 1 {
            return Err(TectonicError::CentroidError { message: "Cannot Descrease Centroid value below 0" })
        }

        for index in 0..D {
            centroid[index] = (centroid[index] * old_n as f32 - vector[index]) * inv_new_avg;
        }

        self.size = new_n;
        Ok(true)
    }

    #[inline]
    pub fn route_to_shard(&mut self, entry: ShardEntry) -> Result<(usize, usize), TectonicError> {
        let hash_value = hash_shard_entry(&entry);
        let length = self.shards.len();

        if length == 0 {
            return Err(TectonicError::RepoError { message: "No Shards initiated in Partition!" });
        }

        let idx1 = (hash_value as usize) % length;
        let idx2 = (secondary_arena_hash(hash_value) as usize) % length;


        let target_index = if idx1 == idx2 {
            idx1
        } else if self.shards[idx1].load_factor <= self.shards[idx2].load_factor {
            idx1
        } else {
            idx2
        };

        let slot_index = self.shards[target_index].insert(entry)?;
        Ok((target_index, slot_index))
    }

    pub fn has_no_centroid(&self) -> bool {
        self.centroid.is_none()
    }

    pub fn has_centroid(&self) -> bool {
        self.centroid.is_some()
    }
}