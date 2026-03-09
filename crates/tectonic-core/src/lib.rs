// ============================================================
// IMPORTS AND MODULES
// ============================================================

mod eviction;
mod metrics;
mod quantization;
mod search;
mod storage;
mod utility;
mod config;
mod error;
mod result;

use crate::config::CacheConfig;
use crate::error::TectonicError;
use crate::metrics::vector_metric::CacheMetrics;
use crate::storage::arena::VectorArena;
use crate::storage::repository::CacheRepo;
use crate::result::DimVector;

// ============================================================
// MAIN CACHE IMPLEMENTATION
// ============================================================

#[allow(dead_code)]
pub struct VectorCache<const D: usize> {
    config: CacheConfig,
    arena: VectorArena<D>,
    repository: CacheRepo<D>,
    metrics: CacheMetrics,
}

impl<const D: usize> VectorCache<D> {
    pub fn new(config: CacheConfig) -> Result<Self, TectonicError> {
        config.validate()?;
        let max_entries = config.max_entries;
        let num_partitions = config.num_partitions;
        let num_shards = config.num_shards;

        Ok(
            Self { 
                config: config, 
                arena: VectorArena::with_capacity(max_entries),
                repository: CacheRepo::with_capacity(max_entries, num_partitions, num_shards),
                metrics: CacheMetrics::default(), 
            }
        )
    }

    pub fn insert(&mut self, vector: DimVector<D>, id: Option<String>, overwrite: bool) -> Result<bool, TectonicError> {
        !todo!()
    }

    pub fn remove(&mut self) -> Result<bool, TectonicError> {
        !todo!()
    }
}

