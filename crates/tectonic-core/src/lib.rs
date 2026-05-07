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
use crate::result::{CacheEntry, CacheResult};
use crate::search::distance::SearchMethod;
use crate::storage::arena::{VectorArena};
use crate::storage::repository::CacheRepo;
use crate::utility::typings::{DimVector, DuplicatePolicy, InsertOutcome, TectonicResult, ValidationMode, usize_to_f32};
use crate::utility::utils::{hash_dimvector, validate_vector};
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
                &self.config.search.distance_metric
            )?;
            

            
        }
        todo!()
    }

    pub fn get<M>(
        &self,
        vector: DimVector<D>,
        search_method: &M,
        k: usize,
        partitions: usize
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

        // 3. Step -> Quantize incomming Vector for faster search.
        let quantized_vector = quantize(&vector)?;

        // 4. Step -> Executre internal search for Top K candidate vectors.
        let search_results = self.repository.search(
            &quantized_vector,
            &vector,
            search_method,
            k,
            partitions
        )?;

        // 5. Step -> Convert the search results into CacheResult with necessary metadata.
        let candidate_count = search_results.len();

        let mut entries = search_results
            .into_iter()
            .map(|result| {
                let arena_position = self.locations.get_location(&result.id).expect("Found nothing!");
                let (candidate, _found_id) = self.arena.get_vector_at_position(*arena_position.get_arena())?;
                let distance = search_method.distance_f32(&vector, candidate);
                
                Ok(CacheEntry::new(*arena_position.get_arena(), candidate.clone(), distance))
            })
            .collect::<Result<Vec<_>, TectonicError>>()?;

        entries.sort_unstable_by(|a, b| a.distance.total_cmp(&b.distance));

        // 6. Step -> Stop latency timer & calculate total execution time.
        let method_latency = time_before_method.elapsed();

        // 7. Step -> Return the final CacheResult instance. 
        Ok(CacheResult::new(k, partitions, candidate_count, method_latency, entries))
    }

    pub fn remove(
        &mut self,
        _internal_id: usize,
    ) -> TectonicResult<DimVector<D>> {
        todo!()
    }

    pub fn extend(
        &mut self,
        _vectors: Vec<DimVector<D>>,
        _overwrite: bool,
    ) -> TectonicResult<bool> {
        todo!()
    }

    pub fn metrics(&self) -> TectonicResult<bool> {
        todo!()
    }

    pub fn config(&self) -> TectonicResult<&CacheConfig> {
        Ok(&self.config)
    }

    pub fn vectors(&self) -> TectonicResult<bool> {
        todo!()
    }

    pub fn is_full(&self) -> TectonicResult<bool> {
        let repo_size = self.repository.size();
        let arena_size = self.arena.size();
        if repo_size != arena_size {
            return Err(TectonicError::arena(
                "Inconsistent size count between Arena and Repository"
            ));
        }
        Ok(self.size()? >= self.metrics.capacity())
    }

    pub fn size(&self) -> TectonicResult<usize> {
        let repo_size = self.repository.size();
        let arena_size = self.arena.size();
        if repo_size == arena_size {
            return Ok(repo_size);
        } else {
            return Err(TectonicError::arena(
                "Inconsistent size count between Arena and Repository"
            ));
        }
    }

    pub fn load_factor(&self) -> f32 {
        usize_to_f32(self.repository.size) / usize_to_f32(self.repository.capacity)
    }

    fn compare_vectors(&self, found_vector: DimVector<D>, new_vector: DimVector<D>) -> bool {
        found_vector == new_vector
    }

// ============================================================
// HELPER METHODS
// ============================================================
}

