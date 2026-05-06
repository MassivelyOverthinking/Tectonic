// ============================================================
// IMPORTS AND MODULES
// ============================================================

use core::f32;
use std::usize;

use crate::error::TectonicError;
use crate::quantization::quantized_entry::QuantizedEntry;
use crate::result::{SearchResult};
use crate::utility::router::BootstrapEntry;
use crate::utility::typings::DimVector;
use crate::search::distance::{SearchMethod};
use crate::location::location_entry::{ShardEntry};
use crate::storage::partition::CachePartition;
use crate::utility::utils::{UniqueID, calculate_sizes, hash_dimvector};
use crate::config::StrategyConfig;

// ============================================================
// INTERNAL STORE (PARTITIONS + SHARDS)
// ============================================================

#[allow(dead_code)]
pub struct CacheRepo<const D: usize> {
    // Main Partition Logic
    pub vector_repo: Vec<CachePartition<D>>,
    pub size: usize,
    pub capacity: usize,

    // Centroid Buffer State
    pub centroid_buffer: Vec<BootstrapEntry<D>>,
    pub centroid_buffer_threshold: usize,
    pub centroids_initialized: bool,
}

#[allow(dead_code)]
impl<const D: usize> CacheRepo<D> {
    pub fn with_capacity(max_entries: usize, partitions: usize, shards: usize, strategy: &StrategyConfig) -> Result<Self, TectonicError> {
        if max_entries == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Cache Max Entries",
                issue: "Maximum entries must be greater than 0",
            });
        }

        if partitions == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Cache Partitions",
                issue: "Number of partitions must be greater than 0",
            });
        }

        if shards == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Cache Shards",
                issue: "Number of shards must be greater than 0",
            });
        }
        
        let partition_capacities = calculate_sizes(max_entries, partitions);
        
        let mut partitions_vector = Vec::with_capacity(partition_capacities.len());
        for (id, &cap) in partition_capacities.iter().enumerate() {
            partitions_vector.push(CachePartition::with_capacity( id as u32, cap as u64, shards as u32, strategy.clone())?);
        }

        let buffer_threshold = (partitions * 16).max(partitions);

        Ok(Self {
            vector_repo: partitions_vector,
            capacity: max_entries,
            size: 0,

            // Buffer State & Initialization
            centroid_buffer: Vec::with_capacity(buffer_threshold),
            centroid_buffer_threshold: buffer_threshold,
            centroids_initialized: false,
        })
    }

    pub fn insert<M>(
        &mut self,
        vector: &DimVector<D>,
        quanttized_vector: QuantizedEntry,
        internal_id: UniqueID,
        distance: &M,
    ) -> Result<bool, TectonicError>
    where M: SearchMethod<D> {
        if !self.centroids_initialized {
            let entry = BootstrapEntry {
                vector: *vector,
                quantized: quanttized_vector,
                internal_id,
                vector_hash: hash_dimvector(vector),
            };

            self.centroid_buffer.push(entry);

            if self.centroid_buffer.len() >= self.centroid_buffer_threshold {
                self.bootstrap_centroids_from_buffer()?;
            }

            return Ok(true);
        }

        self.insert_into_initialized_partitions(vector, quanttized_vector, internal_id, distance)
    }

    pub fn search<M>(
        &self, 
        quanttized_vector: &QuantizedEntry,
        standard_vecor: &DimVector<D>,
        search_method: &M, 
        k: usize,
        search_partitions: usize,
    ) -> Result<Vec<SearchResult>, TectonicError> 
    where M: SearchMethod<D> + Sync, {
        if self.is_empty() {
            return Err(TectonicError::RepoError { message: "Vector repository is currently empty" });
        }

        if k == 0 {
            return Ok(Vec::new());
        }

        if self.vector_repo.is_empty() {
            return Err(TectonicError::RepoError { 
                message: "No internal partitions found!" 
            });
        }

        let partition_budget = search_partitions.min(self.vector_repo.len());
        if partition_budget == 0 {
            return Ok(Vec::new());
        }

        let nearest_partitions = self.find_nearest_centroids(
            standard_vecor,
            partition_budget,
            search_method
        )?;

        let mut merged_results: Vec<SearchResult> = Vec::with_capacity(k);

        for partition_index in nearest_partitions {
            let partition_results = 
            self.vector_repo[partition_index].search(
                quanttized_vector,
                search_method,
                k
            )?;

            if partition_results.is_empty() {
                continue;
            }
            merged_results.extend(partition_results);
            Self::retain_top_k(&mut merged_results, k);

            if merged_results.len() > k {
                return Ok(merged_results);
            }
        }
        Ok(merged_results)
    }

    fn retain_top_k(results: &mut Vec<SearchResult>, k: usize) {
        if results.len() < k {
            return;
        }

        results.sort_unstable();

        if results.len() > k {
            results.truncate(k);
        }
    }

    pub fn get_candidate_partitions(&self, current_tick: u64, target_bytes: usize, k: usize) -> Result<Vec<usize>, TectonicError> {
        if self.vector_repo.is_empty() {
            return Err(TectonicError::RepoError { 
                message: "No internal partitions found!" 
            });
        }

        if target_bytes == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Target partition byte size", 
                issue: "Must be greater than 0" 
            });
        }

        if k == 0 {
            return Ok(Vec::new());
        }

        let limit = k.min(self.vector_repo.len());

        let mut candidates: Vec<(usize, f32)> = Vec::with_capacity(self.vector_repo.len());

        for (position, partition) in self.vector_repo.iter().enumerate() {
            let metrics = partition.metrics();

            #[cfg(debug_assertions)]
            metrics.validate();

            if metrics.is_empty() {
                continue;
            }

            let weakness_score = metrics.weakness_score(current_tick, target_bytes);

            if !weakness_score.is_finite() {
                continue;
            }

            candidates.push((position, weakness_score));
        }

        if candidates.is_empty() {
            return Err(TectonicError::RepoError { 
                message: "No candidate partitions found based on weakness score!" 
            });
        }

        candidates.sort_unstable_by(|x, y| {
            y.1.partial_cmp(&x.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| x.0.cmp(&y.0))
        });

        candidates.truncate(limit);

        Ok(candidates.into_iter().map(|(i, _)| i).collect())
    }

    pub fn find_nearest_centroids<M>(&self, vector: &DimVector<D>, top_n: usize, distance: &M) -> Result<Vec<usize>, TectonicError> 
    where M: SearchMethod<D> {
        
        if self.vector_repo.is_empty() {
            return Err(TectonicError::RepoError { message: "No internal partitions found!" });
        }

        if top_n == 0 {
            return Ok(Vec::new());
        }

        let mut candidates: Vec<(usize, f32)> = Vec::with_capacity(self.vector_repo.len());

        for (position, partition) in self.vector_repo.iter().enumerate() {
            if let Some(par_centroid) = partition.get_centroid() {
                let centroid_distance = distance.distance_f32(vector, &par_centroid);
                candidates.push((position, centroid_distance));
            } else {
                continue;
            }
        }

        
        if candidates.is_empty() {
            return Err(TectonicError::RepoError { message: "No partitions with centroids found!" });
        }

        candidates.sort_unstable_by(|x, y| {
            x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let limit = top_n.min(candidates.len());

        let result = candidates
            .into_iter()
            .take(limit)
            .map(|(position, _)| position)
            .collect();

        Ok(result)
    }

    fn choose_bootstrap_seed_indices(&self, partition_count: usize) -> Result<Vec<usize>, TectonicError> {
        let n = self.centroid_buffer.len();

        if partition_count == 0 {
            return Err(TectonicError::RepoError { message: "Partition count cannot be 0" });
        }

        if n < partition_count {
            return Err(TectonicError::RepoError { message: "Not enough buffered vectors to initialize centroids" });
        }

        let mut seeds = Vec::with_capacity(partition_count);
        let mut min_distances = vec![f32::INFINITY; n];
        let mut chosen = vec![false; n];

        let first_seed = 0usize;
        seeds.push(first_seed);
        chosen[first_seed] = true;

        let first_vector = &self.centroid_buffer[first_seed].vector;
        for i in 0..n {
            let d = Self::squared_l2(&self.centroid_buffer[i].vector, first_vector);
            min_distances[i] = d;
        }
        min_distances[first_seed] = -1.0;

        while seeds.len() < partition_count {
            let mut best_index = usize::MAX;
            let mut best_distance = -1.0_f32;

            for i in 0..n {
                if !chosen[i] && min_distances[i] > best_distance {
                    best_distance = min_distances[i];
                    best_index = i;
                }
            }

            if best_index == usize::MAX {
                return Err(TectonicError::RepoError { message: "Could not determine next bootstrap seed" });
            }

            seeds.push(best_index);
            chosen[best_index] = true;
            min_distances[best_index] = -1.0;

            let seed_vector = &self.centroid_buffer[best_index].vector;
            for i in 0..n {
                if chosen[i] {
                    continue;
                }

                let d = Self::squared_l2(&self.centroid_buffer[i].vector, seed_vector);
                if d < min_distances[i] {
                    min_distances[i] = d;
                }
            }
        }
        Ok(seeds)
    }

    fn assign_buffered_vector_to_seed(&self, vector: &DimVector<D>, seed_indices: &[usize]) -> usize {
        let mut best_partition = 0usize;
        let mut best_distance = f32::INFINITY;

        for (partition_idx, &seed_buffer_index) in seed_indices.iter().enumerate() {
            let seed_vector = &self.centroid_buffer[seed_buffer_index].vector;
            let d = Self::squared_l2(vector, seed_vector);

            if d < best_distance {
                best_distance = d;
                best_partition = partition_idx;
            }
        }

        best_partition
    }

    fn bootstrap_centroids_from_buffer(&mut self) -> Result<bool, TectonicError> {
        if self.centroids_initialized {
            return Ok(false);
        }

        let partition_count = self.vector_repo.len();
        if partition_count == 0 {
            return Err(TectonicError::RepoError { message: "No partitions available for centroid bootstrap" });
        }

        if self.centroid_buffer.len() < partition_count {
            return Err(TectonicError::RepoError { message: "Bootstrap buffer has fewer vectors than partitions" });
        }

        let seed_indices = self.choose_bootstrap_seed_indices(partition_count)?;

        let mut assignments = vec![0usize; self.centroid_buffer.len()];
        let mut counts = vec![0usize; partition_count];
        let mut centroid_sums = vec![[0.0_f32; D]; partition_count];

        // Assignment + accumulation
        for (buffer_index, entry) in self.centroid_buffer.iter().enumerate() {
            let partition_index = self.assign_buffered_vector_to_seed(&entry.vector, &seed_indices);
            assignments[buffer_index] = partition_index;
            counts[partition_index] += 1;

            for dim in 0..D {
                centroid_sums[partition_index][dim] += entry.vector[dim];
            }
        }

        // Initialize partition centroids
        for partition_index in 0..partition_count {
            if counts[partition_index] == 0 {
                let seed_idx = seed_indices[partition_index];
                self.vector_repo[partition_index].set_centroid(self.centroid_buffer[seed_idx].vector)?;
                self.vector_repo[partition_index].set_size(0)?;
                continue;
            }

            let inv = 1.0_f32 / counts[partition_index] as f32;
            let mut centroid = [0.0_f32; D];
            for dim in 0..D {
                centroid[dim] = centroid_sums[partition_index][dim] * inv;
            }

            self.vector_repo[partition_index].set_centroid(centroid)?;
            self.vector_repo[partition_index].set_size(counts[partition_index] as u64)?;
        }

        // Route buffered entries into their assigned partitions/shards
        let drained_entries = std::mem::take(&mut self.centroid_buffer);

        for (buffer_index, entry) in drained_entries.into_iter().enumerate() {
            let partition_index = assignments[buffer_index];

            let internal_id = entry.internal_id;

            let shard_entry = ShardEntry::new(internal_id, entry.quantized);

            let _ = self.vector_repo[partition_index].route_to_shard(shard_entry)?;
        }

        self.centroids_initialized = true;
        Ok(true)
    }

    fn route_partition_for_vector<M>(&self, vector: &DimVector<D>, distance: &M) -> Result<usize, TectonicError>
    where
        M: SearchMethod<D>,
    {
        let nearest = self.find_nearest_centroids(vector, 1, distance)?;
        nearest
            .into_iter()
            .next()
            .ok_or_else(|| TectonicError::RepoError { message: "No centroid route found" })
    }

    fn insert_into_initialized_partitions<M>(
        &mut self,
        vector: &DimVector<D>,
        quantized: QuantizedEntry,
        id: UniqueID,
        distance: &M,
    ) -> Result<bool, TectonicError>
    where
        M: SearchMethod<D>,
    {
        let partition_index = self.route_partition_for_vector(vector, distance)?;

        let shard_entry = ShardEntry::new(id, quantized);

        let _ = self.vector_repo[partition_index].route_to_shard(shard_entry)?;
        self.vector_repo[partition_index].increase_centroid_average(vector)?;

        Ok(true)
    }

    #[inline]
    fn squared_l2(x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        let mut result = 0.0_f32;
        for index in 0..D {
            let distance = x[index] - y[index];
            result += distance * distance;
        }
        result
    }

    #[inline]
    pub fn is_vectors_equal(&self, x: &DimVector<D>, y: &DimVector<D>) -> bool {
        x == y
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
