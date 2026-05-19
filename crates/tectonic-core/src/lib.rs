// ============================================================
// IMPORTS AND MODULES
// ============================================================

mod admission;
mod eviction;
mod location;
mod metrics;
mod quantization;
mod search;
mod storage;
mod utility;
mod config;
mod error;
mod result;

use std::time::Instant;

use crate::config::{CacheConfig};
use crate::error::TectonicError;
use crate::metrics::cache_metrics::CacheMetrics;
use crate::quantization::scalar_qunatization::{quantize};
use crate::result::{CacheEntry, CacheResult, SearchResult, StoredResult};
use crate::search::distance::SearchMethod;
use crate::storage::arena::{VectorArena};
use crate::storage::repository::CacheRepo;
use crate::utility::typings::{DimVector, DuplicatePolicy, InsertOutcome, TectonicResult, ValidationMode, SearchType, usize_to_f32};
use crate::utility::utils::{UniqueID, hash_dimvector, validate_vector};
use crate::location::location_slab::LocationSlab;

// ============================================================
// MAIN CACHE IMPLEMENTATION
// ============================================================

#[allow(dead_code)]
pub struct VectorCache<const D: usize> {
    config: CacheConfig,
    arena: VectorArena<D>,
    repository: CacheRepo<D>,
    locations: LocationSlab,
    metrics: CacheMetrics,
}

impl<const D: usize> VectorCache<D> {
    #[inline]
    pub fn new(config: CacheConfig) -> TectonicResult<Self> {
        config.validate()?;

        let max_entries = config.max_entries;
        let num_partitions = config.num_partitions;
        let num_shards = config.num_shards;

        let arena = VectorArena::with_capacity(max_entries)?;
        let repository = CacheRepo::with_capacity(
            max_entries,
            num_partitions,
            num_shards,
            &config.strategy
        )?;

        let locations = LocationSlab::default();
        let metrics = CacheMetrics::with_capacity(max_entries);

        Ok(
            Self { 
                config, 
                arena, 
                repository, 
                locations, 
                metrics 
            }
        )
    }

    #[inline]
    pub fn insert(
        &mut self, 
        vector: DimVector<D>,
        duplicate: DuplicatePolicy,
        validation_mode: ValidationMode
    ) -> TectonicResult<InsertOutcome> {

        // 1. step => Vector validation
        if matches!(validation_mode, ValidationMode::Strict) {
            validate_vector(&vector)?;
        }

        // 2. step => Duplicate values handling
        let hashed_value = hash_dimvector(&vector);

        // Check for Duplicate using Global Location-Slab
        if let Some(location_entry) = self.locations.get_by_hash(hashed_value) {
            let arena_index = location_entry.get_arena();
            let (found_vector, found_id) = self.arena.get_vector_at_position(*arena_index)?;

            if self.compare_vectors(*found_vector, vector) {
                match duplicate {
                    DuplicatePolicy::ReplaceExisting => {
                        let vector_id = self.arena.update_vector(vector, arena_index)?;
                        return Ok(InsertOutcome::DuplicateReplaced { id: vector_id });
                    },
                    DuplicatePolicy::KeepExisting => {
                        return Ok(InsertOutcome::DuplicateKept { existing: *found_id });
                    },
                }
            }
        }

        // 3. step => Eviction & Insertion
        if !self.is_full()? {

            let (vector_id, arena_index) = self.arena.insert(vector, self.config.metrics.metrics_enabled)?;
            let quantized_vector = quantize(&vector)?;
            let _ = self.repository.insert(
                &vector, 
                quantized_vector, 
                vector_id,
                &self.config.search.distance_metric,
            )?;
            

            
        }
        todo!()
    }

    #[inline]
    pub fn get<M>(
        &self,
        vector: DimVector<D>,
        search_method: &M,
        k: usize,
        partitions: usize,
        search_type: SearchType
    ) -> TectonicResult<CacheResult<D>> 
    where M: SearchMethod<D>{
        // 1. Step -> Check if main cahce is currently empty.
        if self.metrics.is_empty() {
            return Err(TectonicError::repository(
                "No vectors currently stored in Repository"
            ));
        }

        // 2. Step -> Start lantency timer for search-method execution.
        let time_before_method = Instant::now();

        // 3. Step -> Validate and execute search type.
        match search_type {
            // Exact similarity using non-quantized vectors for high accuracy.
            SearchType::Accurate => {
                let accurate_results = self.arena.accurate_search(vector, k, search_method)?;

                // Final length of candidate vectors returned from accurate search.
                let candidate_count = accurate_results.len();

                // Stop latency timer & calculate total execution time.
                let method_latency = time_before_method.elapsed();

                // Return the final CacheResult instance. 
                Ok(CacheResult::new(k, partitions, candidate_count, method_latency, accurate_results))
            },
            SearchType::Approximate => {
                // Approximate similarity using quantized vectors for faster search.
                // Calculate quantized vector for incoming query.
                let quantized_vector = quantize(&vector)?;

                // Execute internal search for Top K candidate vectors.
                let search_results = self.repository.search(
                    &quantized_vector,
                    &vector,
                    search_method,
                    k,
                    partitions
                )?;

                // Final length of candidate vectors returned from approximate search.
                let candidate_count = search_results.len();

                // Convert the candiate results into finalized CacheResult entries.
                let entries = self.convert_search_results(search_results, vector, search_method)?;

                // Stop latency timer & calculate total execution time.
                let method_latency = time_before_method.elapsed();

                // Return the final CacheResult instance. 
                Ok(CacheResult::new(k, partitions, candidate_count, method_latency, entries))
            },
            SearchType::ApproximateRerank => {
                // Aprroximate similarity search without final reranking step.
                // Calculate quantized vector for incoming query.
                let quantized_vector = quantize(&vector)?;

                // Execute internal search for Top K candidate vectors.
                let search_results = self.repository.search(
                    &quantized_vector,
                    &vector,
                    search_method,
                    k,
                    partitions
                )?;

                // Final length of candidate vectors returned from approximate search.
                let candidate_count = search_results.len();

                // Convert the candiate results into finalized CacheResult entries.
                // And perform a final reranking step.
                let mut entries = self.convert_search_results(search_results, vector, search_method)?;
                entries.sort_unstable_by(|a, b| a.get_distance().total_cmp(&b.get_distance()));

                // Stop latency timer & calculate total execution time.
                let method_latency = time_before_method.elapsed();

                // Return the final CacheResult instance.
                Ok(CacheResult::new(k, partitions, candidate_count, method_latency, entries))
            },
        }
    }

    #[inline]
    pub fn contains(&self, vector: DimVector<D>) -> TectonicResult<bool> {

        // 1. Step -> Hash the incoming vector value.
        let hashed_value = hash_dimvector(&vector);

        // 2. Step -> Check Global Location Slab for existing hash value.
        if let Some(location_entry) = self.locations.get_by_hash(hashed_value) {
            // 3. Step -> If hash exists, retrieve the correct arena index.
            let arena_index = location_entry.get_arena();

            // 4. Step -> Retieve the vector from arena and make final comparison.
            let (found_vector, _found_id) = self.arena.get_vector_at_position(*arena_index)?;
            Ok(self.compare_vectors(*found_vector, vector))
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub fn remove(
        &mut self,
        id: UniqueID,
        force: bool,
    ) -> TectonicResult<DimVector<D>> {    
        // 1. Step -> Start lantency timer for removal execution.
        let time_before_method = Instant::now();

        // 2. Step -> Retrive Location-entry from global Location Arena/Slab.
        let location = self
            .locations
            .get_location(&id)
            .ok_or_else(|| TectonicError::location("No Location entry found for requested ID"))
            .clone()?;

        // 3. Step ->
        if location.is_pending() && !force {
            return Err(TectonicError::location(
                "Cannot remove 'Pending' vector - Leverage 'Force' parameter to enable this behaviour"
            ));
        };

        let arena_index = *location.get_arena();

        let (found_vector, found_id) = self.arena.get_vector_at_position(arena_index)?;

        if *found_id != id {
            return Err(TectonicError::inconsistent_state(
                "Location Slab index doesn't match requested ID"
            ));
        };

        let removed_vector = *found_vector;

        self.repository.remove(
            location.get_partition()?, 
            location.get_shard()?, 
            location.get_slot()?, 
            id, 
            &removed_vector
        )?;

        self.arena.remove(id, arena_index)?;

        self.locations.remove(&id)?;

        let time_after_method = time_before_method.elapsed();

        self.metrics.on_remove(time_after_method);

        Ok(removed_vector)
    }

    #[inline]
    pub fn extend(
        &mut self,
        _vectors: Vec<DimVector<D>>,
        _overwrite: bool,
    ) -> TectonicResult<bool> {
        todo!()
    }

    #[inline]
    pub fn metrics(&self) -> TectonicResult<bool> {
        todo!()
    }

    #[inline]   
    pub fn config(&self) -> TectonicResult<&CacheConfig> {
        Ok(&self.config)
    }

    #[inline]
    pub fn vectors(&self) -> TectonicResult<Vec<StoredResult<D>>> {
        let size = self.size()?;

        if size == 0 {
            return Ok(Vec::new());
        };

        let mut elements: Vec<StoredResult<D>> = Vec::with_capacity(size);

        for (index, slot) in self.arena.slots().iter().enumerate() {
            let Some(entry) = slot.vector.as_ref() else {
                continue;
            };
            
            #[cfg(debug_assertions)]
            {
                let location = self
                    .locations
                    .get_location(&entry.get_unique_id())
                    .ok_or_else(|| {
                        TectonicError::inconsistent_state(
                            "Arena vector has no matching Location entry")
                    })?;

                debug_assert_eq!(
                    *location.get_arena(),
                    index,
                    "Location Arena/Slab index does not match arena position"
                );
            }

            elements.push(StoredResult::new(
                *entry.get_vector(), 
                *entry.get_unique_id()
            ));
        }

        if size != elements.len() {
            return Err(TectonicError::inconsistent_state(
                "Output array length doesn't match correct Arena size"
            ));
        };

        Ok(elements)
    }

    #[inline]
    pub fn is_full(&self) -> TectonicResult<bool> {
        let repo_size = self.repository.size();
        let arena_size = self.arena.size();
        if repo_size != arena_size {
            return Err(TectonicError::internal_mismatch(
                "Inconsistent size count between Arena and Repository"
            ));
        }
        Ok(self.size()? >= self.metrics.capacity())
    }

    #[inline]
    pub fn size(&self) -> TectonicResult<usize> {
        let repo_size = self.repository.size();
        let arena_size = self.arena.size();
        if repo_size == arena_size {
            return Ok(repo_size);
        } else {
            return Err(TectonicError::internal_mismatch(
                "Inconsistent size count between Arena and Repository"
            ));
        }
    }

    #[inline]
    pub fn clear(&mut self) -> TectonicResult<bool> {
        self.arena.clear()?;
        self.repository.clear()?;
        self.locations.clear();
        self.metrics.reset();

        Ok(true)
    }

    #[inline]
    pub fn load_factor(&self) -> f32 {
        usize_to_f32(self.repository.size) / usize_to_f32(self.repository.capacity)
    }

    #[inline]
    fn compare_vectors(&self, found_vector: DimVector<D>, new_vector: DimVector<D>) -> bool {
        found_vector == new_vector
    }

// ============================================================
// HELPER METHODS
// ============================================================

    #[inline]
    fn convert_search_results<M>(&self, results: Vec<SearchResult>, vector: DimVector<D>, search_method: &M)
    -> TectonicResult<Vec<CacheEntry<D>>> where M: SearchMethod<D> {
        let entries = results
            .into_iter()
            .map(|result| {
                let arena_position = self.locations.get_location(&result.id).expect("Found nothing!");
                let (candidate, _found_id) = self.arena.get_vector_at_position(*arena_position.get_arena())?;
                let distance = search_method.distance_f32(&vector, candidate);
                
                Ok(CacheEntry::new(*arena_position.get_arena(), candidate.clone(), distance))
            })
            .collect::<Result<Vec<_>, TectonicError>>()?;

        Ok(entries)
    }
}

