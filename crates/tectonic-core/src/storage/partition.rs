// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::BinaryHeap;
use rayon::prelude::*;

use crate::config::{StrategyConfig, StrategyStructure};
use crate::error::TectonicError;
use crate::metrics::cluster_metrics::ClusterMetrics;
use crate::quantization::quantized_entry::QuantizedEntry;
use crate::result::{MergeResult, SearchResult};
use crate::search::distance::{SearchMethod};
use crate::utility::typings::{DimVector, TectonicResult};
use crate::storage::shard::{CacheShard};
use crate::location::location_entry::{ShardEntry};
use crate::utility::utils::{UniqueID, calculate_sizes, hash_shard_entry, secondary_arena_hash};

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    partition_id: u32,
    centroid: Option<[f32; D]>,
    size: u64,
    capacity: u64,
    shards: Vec<CacheShard<D>>,
    strategy: StrategyStructure,

    // Internal Partition-metrics.
    metrics: ClusterMetrics
}

impl<const D: usize> CachePartition<D> {
    pub fn with_capacity(partition_id: u32, capacity: u64, num_shards: u32, strategy: StrategyConfig) -> TectonicResult<Self> {
        let shard_sizes = calculate_sizes(capacity as usize, num_shards as usize);

        let mut shard_vectors = Vec::with_capacity(shard_sizes.len());
        for (id, &cap) in shard_sizes.iter().enumerate() {
            shard_vectors.push(CacheShard::with_capacity(id as u32, cap));
        }

        Ok(Self { 
            partition_id,
            centroid: None,
            size: 0, 
            capacity, 
            shards: shard_vectors,
            strategy: StrategyStructure::from_config(strategy.clone())?,
            metrics: ClusterMetrics::default(),
        })
    }

    pub fn search<M>(
        &self,
        vector: &QuantizedEntry,
        search_method: &M,
        k: usize
    ) -> TectonicResult<Vec<SearchResult>>
    where M: SearchMethod<D> + Sync {
        if k == 0 || self.shards.is_empty() {
            return Ok(Vec::new());
        }

        let shard_results: TectonicResult<Vec<Vec<SearchResult>>> = self
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
    pub fn increase_centroid_average(&mut self, vector: &DimVector<D>) ->TectonicResult<bool> {
        let centroid = self.centroid.as_mut().ok_or_else(|| {
            TectonicError::centroid("No centroid available!")
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
    pub fn remove_from_shard(
        &mut self, 
        shard: usize, 
        slot: usize,
        id: UniqueID,
        vector: &DimVector<D>
    ) -> TectonicResult<bool> {
        let shard = self
            .shards
            .get_mut(shard)
            .ok_or_else(|| TectonicError::repository("Shard index ouf of bounds"))?;

        shard.remove(slot, id)?;

        self.decrease_centroid_average(vector)?;

        Ok(true)
    }

    #[inline]
    pub fn decrease_centroid_average(&mut self, vector: &DimVector<D>) -> TectonicResult<bool> {
        let centroid = self.centroid.as_mut().ok_or_else(|| {
            TectonicError::centroid("No centroid available!")
        })?;

        let old_n = self.size;
        let new_n = old_n - 1;
        let inv_new_avg = 1.0f32 / (new_n as f32);

        if old_n <= 1 {
            return Err(TectonicError::centroid("Cannot decrease Centroid value below 0"))
        }

        for index in 0..D {
            centroid[index] = (centroid[index] * old_n as f32 - vector[index]) * inv_new_avg;
        }

        self.size = new_n;
        Ok(true)
    }

    #[inline]
    pub fn route_to_shard(&mut self, entry: ShardEntry) -> TectonicResult<(usize, usize)> {
        let hash_value = hash_shard_entry(&entry);
        let length = self.shards.len();

        if length == 0 {
            return Err(TectonicError::repository("No Shards initiated in Partition!"));
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

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    #[inline]
    pub fn increment_size(&mut self) -> TectonicResult<()> {
        if self.size >= self.capacity {
            return Err(TectonicError::repository("Partition capacity exceeded!"));
        }

        self.size += 1;
        Ok(())
    }

    #[inline]
    pub fn set_size(&mut self, size: u64) -> TectonicResult<()> {
        if size > self.capacity {
            return Err(TectonicError::repository("Partition capacity exceeded!"));
        }

        self.size = size;
        Ok(())
    }

    #[inline]
    pub fn has_no_centroid(&self) -> bool {
        self.centroid.is_none()
    }

    #[inline]
    pub fn has_centroid(&self) -> bool {
        self.centroid.is_some()
    }

    #[inline]
    pub fn get_partition_id(&self) -> u32 {
        self.partition_id
    }

    #[inline]
    pub fn centroid(&self) -> Option<&DimVector<D>> {
        self.centroid.as_ref()
    }

    #[inline]
    pub fn metrics(&self) -> &ClusterMetrics {
        &self.metrics
    }

    #[inline]
    pub fn metrics_mut(&mut self) -> &mut ClusterMetrics {
        &mut self.metrics
    }

    #[inline]
    pub fn set_centroid(&mut self, centroid: DimVector<D>) -> TectonicResult<bool> {
        self.centroid = Some(centroid);
        Ok(true)
    }

    #[inline]
    pub fn clear_centroid(&mut self) {
        self.centroid = None;
    }

    #[inline]
    pub fn get_centroid(&self) -> Option<DimVector<D>> {
        self.centroid
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    #[inline]
    pub fn clear(&mut self) -> TectonicResult<bool> {
        for shard in self.shards.iter_mut() {
            shard.clear()?;
        };

        self.centroid = None;
        self.size = 0;
        Ok(true)
    }

    #[inline]
    pub fn insert_admission(&mut self, id: UniqueID) -> TectonicResult<bool> {
        self.strategy.get_admission_mut().on_insert(&id);
        Ok(true)
    }
}