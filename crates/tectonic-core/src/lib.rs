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

#[allow(dead_code)]
pub struct VectorCache<const D: usize> {
    // Empty shell -> Initialization handled by .builder-pattern & CacheConfig
}

