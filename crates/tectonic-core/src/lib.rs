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
use crate::metrics::cache_metrics::CacheMetrics;
use crate::result::VectorResult;
use crate::storage::arena::VectorArena;
use crate::storage::repository::CacheRepo;
use crate::utility::typings::{DimVector, usize_to_f32};

// ============================================================
// MAIN CACHE IMPLEMENTATION
// ============================================================

#[allow(dead_code)]
pub struct VectorCache<'a, const D: usize> {
    config: CacheConfig,
    arena: VectorArena<'a, D>,
    repository: CacheRepo<D>,
    metrics: CacheMetrics,
}

impl<'a, const D: usize> VectorCache<'a, D> {
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
                metrics: CacheMetrics::with_capacity(max_entries), 
            }
        )
    }

    pub fn insert(
        &mut self, 
        _vector: DimVector<D>,
        _id: Option<String>,
        _overwrite: bool
    ) -> Result<bool, TectonicError> {
        todo!()
    }

    pub fn get(
        &self,
        _vector: DimVector<D>,
        _k: usize,
        _partitions: usize
    ) -> Result<VectorResult<D>, TectonicError> {
        todo!()
    }

    pub fn remove(
        &mut self,
        _user_id: Option<String>,
        _internal_id: usize,
    ) -> Result<DimVector<D>, TectonicError> {
        todo!()
    }

    pub fn extend(
        &mut self,
        _vectors: Vec<DimVector<D>>,
        _overwrite: bool,
    ) -> Result<bool, TectonicError> {
        todo!()
    }

    pub fn metrics(&self) -> Result<bool, TectonicError> {
        todo!()
    }

    pub fn config(&self) -> Result<bool, TectonicError> {
        todo!()
    }

    pub fn vectors(&self) -> Result<bool, TectonicError> {
        todo!()
    }

    pub fn is_full(&self) -> bool {
        self.repository.is_full()
    }

    pub fn size(&self) -> usize {
        self.repository.size
    }

    pub fn load_factor(&self) -> f32 {
        usize_to_f32(self.repository.size) / usize_to_f32(self.repository.capacity)
    }
}

