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

// ============================================================
// MAIN CACHE IMPLEMENTATION
// ============================================================

#[allow(dead_code)]
pub struct VectorCache<const D: usize> {
    config: CacheConfig,
}

impl<const D: usize> VectorCache<D> {
    pub fn new(config: CacheConfig) -> Result<Self, TectonicError> {
        config.validate()?;

        Ok(Self { config })
    }
}

