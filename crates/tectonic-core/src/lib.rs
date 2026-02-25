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
use crate::storage::arena::VectorArena;

// ============================================================
// MAIN CACHE IMPLEMENTATION
// ============================================================

#[allow(dead_code)]
pub struct VectorCache<const D: usize> {
    config: CacheConfig,
    arena: VectorArena<D>,
}

impl<const D: usize> VectorCache<D> {
    pub fn new(config: CacheConfig) -> Result<Self, TectonicError> {
        config.validate()?;
        let max_entries = config.max_entries;

        Ok(
            Self { 
                config: config, 
                arena: VectorArena::with_capacity(max_entries) 
            }
        )
    }
}

