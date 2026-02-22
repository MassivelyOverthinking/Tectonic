// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::error::TectonicError;

// ============================================================
// INTERNAL CONFIGURATION OBJECTS
// ============================================================
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub num_partitions: usize,
    pub num_shards: usize,
    pub quantization_enabled: bool,
    pub search: SearchConfig,
    pub eviction: EvictionConfig,
    pub routing: RoutingConfig,
    pub maintenance: MaintenanceConfig,
    pub metrics: MetricsConfig,
}

impl CacheConfig {
    // Validation-method for CacheConfig to ensure parameter integrity.
    pub fn validate(&self) -> Result<(), TectonicError> {
        if self.max_entries <= 0 { 
            return Err(TectonicError::new("Max Entries must be a positive integer!")); 
        }
        if self.num_partitions <= 0 {
            return Err(TectonicError::new("Number of partitions must be a positive integer")); 
        }
        if self.num_shards <= 0 {
            return Err(TectonicError::new("Number of shard must be a positive integer"));
        }
        if self.routing.search_partitions <= 0 {
            return Err(TectonicError::new("Search partitions must be a positive integer"));
        }
        if self.routing.search_partitions > self.num_partitions {
            return Err(TectonicError::new("Search partitions must not be greater than number of current partitions")); 
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub distance_metric: None,
    pub similarity_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct EvictionConfig {
    pub eviction_strategy: None,
}

#[derive(Debug, Clone)]
pub struct RoutingConfig {
    search_partitions: usize
}

#[derive(Debug, Clone)]
pub struct MaintenanceConfig {
    coopoerative: bool,
    hysteresis: f32,
    move_cooldown: usize,
    step_cooldown: usize,
}

#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub metrics_enabled: bool
}

