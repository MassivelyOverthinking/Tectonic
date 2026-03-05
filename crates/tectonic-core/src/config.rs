// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::error::TectonicError;
use crate::search::distance::DistanceMetric;
use crate::eviction::eviction_strategy::{Eviction};

// ============================================================
// INTERNAL CONFIGURATION OBJECTS
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[allow(dead_code)]
impl CacheConfig {
    // Entry point for VectorCache -builder functionality
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder::default()
    }

    // Validation-method for CacheConfig to ensure parameter integrity.
    pub fn validate(&self) -> Result<(), TectonicError> {
        if self.max_entries <= 0 { 
            return Err(TectonicError::InvalidParamaterError { param: "Max Entries", issue: "be a Positive Integer" }); 
        }
        if self.num_partitions <= 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Number of partitions", issue: "be a Positive Integer" }); 
        }
        if self.num_shards <= 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Number of shards", issue: "be a Positive Integer" });
        }
        if self.routing.search_partitions <= 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Search partitions", issue: "be a Positive Integer" });
        }
        if self.routing.search_partitions > self.num_partitions {
            return Err(TectonicError::InvalidParamaterError { param: "Search partitions", issue: "not be greater than number of partitions" }); 
        }
        Ok(())
    }
}

// ============================================================
// INTERNAL BUILDER
// ============================================================

#[derive(Default)]
#[allow(dead_code)]
pub struct CacheConfigBuilder {
    max_entries: Option<usize>,
    num_partitions: Option<usize>,
    num_shards: Option<usize>,
    quantization_enabled: Option<bool>,
    distance_metric: Option<DistanceMetric>,
    similarity_threshold: Option<f32>,
    eviction_strategy: Option<Eviction>,
    search_partitions: Option<usize>,
    coopoerative: Option<bool>,
    hysteresis: Option<f32>,
    move_cooldown: Option<usize>,
    step_cooldown: Option<usize>,
    metrics_enabled: Option<bool>,
}

#[allow(dead_code)]
impl CacheConfigBuilder {
    pub fn max_entries(mut self, value: usize) -> Self { 
        self.max_entries = Some(value); 
        self
    }

    pub fn num_partitions(mut self, value: usize) -> Self {
        self.num_partitions = Some(value);
        self
    }

    pub fn num_shards(mut self, value: usize) -> Self {
        self.num_shards = Some(value);
        self
    }

    pub fn quantization_enabled(mut self, value: bool) -> Self {
        self.quantization_enabled = Some(value);
        self
    }

    pub fn distance_metric(mut self, value: DistanceMetric) -> Self {
        self.distance_metric = Some(value);
        self
    }

    pub fn eviction_strategy(mut self, value: Eviction) -> Self {
        self.eviction_strategy = Some(value);
        self
    }

    pub fn search_partitions(mut self, value: usize) -> Self {
        self.search_partitions = Some(value);
        self
    }

    pub fn coopoerative(mut self, value: bool) -> Self {
        self.coopoerative = Some(value);
        self
    }

    pub fn hysteresis(mut self, value: f32) -> Self {
        self.hysteresis = Some(value);
        self
    }

    pub fn move_cooldown(mut self, value: usize) -> Self {
        self.move_cooldown = Some(value);
        self
    }

    pub fn step_cooldown(mut self, value: usize) -> Self {
        self.step_cooldown = Some(value);
        self
    }

    pub fn metrics_enabled(mut self, value: bool) -> Self {
        self.metrics_enabled = Some(value);
        self
    }

    pub fn build(self) -> Result<CacheConfig, TectonicError> {
        let max_entries = self
            .max_entries
            .ok_or_else(|| TectonicError::RequiredFieldError { field: "Max Entries" })?;
        let num_partitions = self
            .num_partitions
            .ok_or_else(|| TectonicError::RequiredFieldError { field: "Number of Partitions" })?;

        const DEFAULT_SHARDS: usize = 1;
        const DEFAULT_DISTANCE_METRIC: DistanceMetric = DistanceMetric::Euclidean;
        const DEFAULT_SIM_THRESHOLD: f32 = 0.0;
        const DEFAULT_EVICTION: Eviction = Eviction::FIFO;
        const DEFAULT_SEARCH_PARTITIONS: usize = 3;
        const DEFAULT_COOPOERATIVE: bool = true;
        const DEFAULT_HYSTERESIS: f32 = 0.2;
        const DEFAULT_MOVE_COOLDOWN: usize = 10_000;
        const DEFAULT_STEP_COOLDOWN: usize = 0;
        const DEFAULT_METRICS_ENABLED: bool = true;
        const DEFAULT_QUANTIZATION_ENABLED: bool = false;

        let num_shards = self.num_shards.unwrap_or(DEFAULT_SHARDS);
        let quantization_enabled = self.quantization_enabled.unwrap_or(DEFAULT_QUANTIZATION_ENABLED);

        let search_config = SearchConfig {
            distance_metric: self.distance_metric.unwrap_or(DEFAULT_DISTANCE_METRIC),
            similarity_threshold: self.similarity_threshold.unwrap_or(DEFAULT_SIM_THRESHOLD)
        };

        let eviction_config = EvictionConfig {
            eviction_strategy: self.eviction_strategy.unwrap_or(DEFAULT_EVICTION)
        };

        let routing_config = RoutingConfig {
            search_partitions: self.search_partitions.unwrap_or(DEFAULT_SEARCH_PARTITIONS)
        };

        let maintenance_config = MaintenanceConfig {
            coopoerative: self.coopoerative.unwrap_or(DEFAULT_COOPOERATIVE),
            hysteresis: self.hysteresis.unwrap_or(DEFAULT_HYSTERESIS),
            move_cooldown: self.move_cooldown.unwrap_or(DEFAULT_MOVE_COOLDOWN),
            step_cooldown: self.step_cooldown.unwrap_or(DEFAULT_STEP_COOLDOWN)
        };

        let metrics_config = MetricsConfig {
            metrics_enabled: self.metrics_enabled.unwrap_or(DEFAULT_METRICS_ENABLED)
        };

        let cache_config = CacheConfig {
            max_entries: max_entries,
            num_partitions: num_partitions,
            num_shards: num_shards,
            quantization_enabled: quantization_enabled,
            search: search_config,
            eviction: eviction_config,
            routing: routing_config,
            maintenance: maintenance_config,
            metrics: metrics_config
        };

        cache_config.validate()?;
        Ok(cache_config)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SearchConfig {
    pub distance_metric: DistanceMetric,
    pub similarity_threshold: f32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EvictionConfig {
    pub eviction_strategy: Eviction,
}

#[derive(Debug, Clone)]
pub struct RoutingConfig {
    search_partitions: usize
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MaintenanceConfig {
    coopoerative: bool,
    hysteresis: f32,
    move_cooldown: usize,
    step_cooldown: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MetricsConfig {
    pub metrics_enabled: bool
}

