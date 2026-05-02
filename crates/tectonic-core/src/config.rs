// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::error::TectonicError;
use crate::search::distance::DistanceMetric;
use crate::eviction::eviction_strategy::{Eviction, EvictionStrategy};
use crate::admission::admission_strategy::{Admission, AdmissionStrategy};
use crate::admission::always_admission::AlwaysAdmission;
use crate::admission::twohit_admission::TwoHitAdmission;
use crate::admission::tinylfu_admission::TinyLFUAdmission;
use crate::admission::windowlfu_admission::WindowTinyLFUAdmisssion;
use crate::eviction::partitioned_fifo::PartitionedFIFO;
use crate::eviction::partitioned_lifo::PartitionedLIFO;
use crate::eviction::partitioned_lru::PartitionedLRU;
use crate::eviction::segmented_lru::SegmentedLRU;
use crate::eviction::varc::VARC;

// ============================================================
// INTERNAL CONFIGURATION OBJECTS
// ============================================================
// Configuration, Building & Stragety construction for main Vector Cache
// ---
// This module owns the initialization path for `VectorCache`.
// It converts user-facing builder parameters into validated internal
// configuration objects and constructs admission/eviction strategy
// implementations from those configurations.
// ---
// The configuration layer holds 3 main responsibilities:
// 1. Provide a stable and reliable Builder API for cache initialization.
// 2. Perform parameter validation before cache initialization.
// 3. Materialize complete function-ready vector cache object.
// ---
// All paramater validation is performed before cache-object initialization
// so that invalid runtime states are rejected rearly and consistently.


// Fully resolved configuration object for main `VectorCache`.
// ---
// `CacheConfig` is the immutable runtime configuration produced by
// `CacheConfigBuilder`. All optional builder parameters are resolved into
// concrete values before this type is returned.
// ---
// This type should be treated as the single source of truth during cache
// initialization. Any component that depends on capacity, routing, search,
// maintenance, metrics, admission, or eviction behavior should read from this
// object instead of duplicating configuration state.
// ---
// Invariants:
// - `max_entries > 0`
// - `num_partitions > 0`
// - `num_shards > 0`
// - `routing.search_partitions > 0`
// - `routing.search_partitions <= num_partitions`
// - `strategy.capacity <= max_entries`
// - `strategy.validate()` succeeds
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub num_partitions: usize,
    pub num_shards: usize,
    pub search: SearchConfig,
    pub strategy: StrategyConfig,
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

    // Validates the fully resolved cache configuration.
    // ---
    // This method enforces cross-field invariants that cannot be checked by
    // individual builder setters. It should be called exactly once before
    // constructing runtime cache structures.
    // ---
    // Validation is intentionally strict: invalid configuration should fail during
    // initialization rather than causing degraded recall, unbounded memory growth,
    // panics, or inconsistent eviction behavior later.
    pub fn validate(&self) -> Result<(), TectonicError> {
        if self.max_entries == 0 { 
            return Err(TectonicError::InvalidParamaterError { param: "Max Entries", issue: "be a Positive Integer" }); 
        }
        if self.num_partitions == 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Number of partitions", issue: "be a Positive Integer" }); 
        }
        if self.num_shards == 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Number of shards", issue: "be a Positive Integer" });
        }
        if self.routing.search_partitions == 0 {
            return Err(TectonicError::InvalidParamaterError { param: "Search partitions", issue: "be a Positive Integer" });
        }
        if self.routing.search_partitions > self.num_partitions {
            return Err(TectonicError::InvalidParamaterError { param: "Search partitions", issue: "not be greater than number of partitions" }); 
        }

        if self.strategy.capacity > self.max_entries {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Strategy Capacity", 
                issue: "Must not exceed Max Entries" 
            });
        }

        self.strategy.validate()?;

        Ok(())
    }

    pub fn debug_summary(&self) -> String {
        format!(
            "CacheConfig {{ max_entries: {}, partitions: {}, shards: {}, search_partitions: {}, admission: {:?}, eviction: {:?}, strategy_capacity: {}, metrics: {} }}",
            self.max_entries,
            self.num_partitions,
            self.num_shards,
            self.routing.search_partitions,
            self.strategy.admission_policy,
            self.strategy.eviction_policy,
            self.strategy.capacity,
            self.metrics.metrics_enabled,
        )
    }
}

// ============================================================
// INTERNAL BUILDER
// ============================================================

// Builder pattern constructor for producing a validated `CacheConfig`.
// ---
// The Builder-pattern separates required parameters from tunable defaults.
// `max_entries` and `num_partitions` are required because they define the
// physical cache size and partitioning model. Other fields have production
// defaults suitable for a high-throughput cache.
// ---
// Validation is deferred until `build()` so callers can configure fields in
// any order.
// ---
// Default strategy:
// - Admission: `Admission::WeightedTinyLFU`
// - Eviction: `Eviction::VARC`
// - Strategy capacity: `max_entries`
// - Search partitions: `3`
// - Shards: `1`
#[derive(Default)]
#[allow(dead_code)]
pub struct CacheConfigBuilder {
    max_entries: Option<usize>,
    num_partitions: Option<usize>,
    num_shards: Option<usize>,
    
    distance_metric: Option<DistanceMetric>,
    similarity_threshold: Option<f32>,

    admission_strategy: Option<Admission>,
    eviction_strategy: Option<Eviction>,
    strategy_capacity: Option<usize>,
    admission_threshold: Option<f32>,
    admission_width: Option<usize>,
    admission_depth: Option<usize>,
    admission_frequency: Option<u8>,
    window_capacity: Option<usize>,

    search_partitions: Option<usize>,
    cooperative: Option<bool>,

    hysteresis: Option<f32>,
    move_cooldown: Option<usize>,
    step_cooldown: Option<usize>,
    metrics_enabled: Option<bool>,
}

#[allow(dead_code)]
impl CacheConfigBuilder {
    // Sets the maximum number of vector entries the cache may hold.
    // ---
    // This is the primary capacity bound for the cache. Unless
    // `strategy_capacity()` is explicitly set, this value is also used as the
    // capacity for the admission/eviction strategy layer.
    pub fn max_entries(mut self, value: usize) -> Self { 
        self.max_entries = Some(value); 
        self
    }

    // Sets the number of IVF-style partitions used for vector routing.
    // ---
    // More partitions can reduce candidate-set size during lookup, but may
    // increase maintenance overhead and routing complexity.
    pub fn num_partitions(mut self, value: usize) -> Self {
        self.num_partitions = Some(value);
        self
    }

    // Sets the number of independent shards used by the cache.
    // ---
    // Sharding can improve write/read concurrency, but excessive shard counts may
    // increase memory overhead and reduce per-shard locality.
    pub fn num_shards(mut self, value: usize) -> Self {
        self.num_shards = Some(value);
        self
    }

    pub fn distance_metric(mut self, value: DistanceMetric) -> Self {
        self.distance_metric = Some(value);
        self
    }

    // Sets the admission policy used to decide whether a candidate should enter the cache.
    // ---
    // TinyLFU-style policies are better for protecting the cache from one-off
    // accesses, while `Always` is simpler and cheaper but less selective.
    pub fn admission_strategy(mut self, value: Admission) -> Self {
        self.admission_strategy = Some(value);
        self
    }

    // Sets the eviction policy used when the cache is at capacity.
    // ---
    // The eviction policy determines which resident item is removed when a new
    // item is admitted and capacity has been reached.
    pub fn eviction_strategy(mut self, value: Eviction) -> Self {
        self.eviction_strategy = Some(value);
        self
    }

    // Sets the capacity used by the admission/eviction strategy layer.
    // ---
    // This defaults to `max_entries`. It should usually not exceed `max_entries`,
    // because the strategy layer should not believe it can retain more entries
    // than the cache itself can store.    
    pub fn strategy_capacity(mut self, value: usize) -> Self {
        self.strategy_capacity = Some(value);
        self
    }

    pub fn admission_threshold(mut self, value: f32) -> Self {
        self.admission_threshold = Some(value);
        self
    }

    // Sets the width of the TinyLFU frequency sketch.
    // ---
    // Larger widths reduce hash collisions and improve frequency estimation
    // accuracy, but consume more memory.
    pub fn admission_width(mut self, value: usize) -> Self {
        self.admission_width = Some(value);
        self
    }

    // Sets the depth of the TinyLFU frequency sketch.
    // --- 
    // Larger depths improve robustness against collisions, but increase update
    // cost and memory usage.
    pub fn admission_depth(mut self, value: usize) -> Self {
        self.admission_depth = Some(value);
        self
    }

    // Sets the minimum frequency threshold used by TinyLFU-style admission.
    // ---
    // This controls how aggressively the admission layer rejects low-frequency
    // items. The value must be finite.
    pub fn admission_frequency(mut self, value: u8) -> Self {
        self.admission_frequency = Some(value);
        self
    }

    // Sets the size of the admission window used by windowed TinyLFU policies.
    // ---
    // The window allows new items to prove utility before competing with the main
    // cache. It must be greater than zero and must not exceed strategy capacity.
    pub fn window_capacity(mut self, value: usize) -> Self {
        self.window_capacity = Some(value);
        self
    }

    // Sets how many partitions are searched per lookup.
    // ---
    // Higher values improve recall but increase query latency. This value must be
    // greater than zero and must not exceed `num_partitions`.
    pub fn search_partitions(mut self, value: usize) -> Self {
        self.search_partitions = Some(value);
        self
    }

    pub fn cooperative(mut self, value: bool) -> Self {
        self.cooperative = Some(value);
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
        const DEFAULT_SEARCH_PARTITIONS: usize = 3;
        const DEFAULT_COOPERATIVE: bool = true;
        const DEFAULT_HYSTERESIS: f32 = 0.2;
        const DEFAULT_MOVE_COOLDOWN: usize = 10_000;
        const DEFAULT_STEP_COOLDOWN: usize = 0;
        const DEFAULT_METRICS_ENABLED: bool = true;
        const DEFAULT_ADMISSON_STRATEGY: Admission = Admission::WeightedTinyLFU;
        const DEFAULT_EVICTION_STRATEGY: Eviction = Eviction::VARC;

        let num_shards = self.num_shards.unwrap_or(DEFAULT_SHARDS);

        let search_config = SearchConfig {
            distance_metric: self.distance_metric.unwrap_or(DEFAULT_DISTANCE_METRIC),
            similarity_threshold: self.similarity_threshold.unwrap_or(DEFAULT_SIM_THRESHOLD)
        };

        let mut strategy_config = StrategyConfig::new(
            self.admission_strategy.unwrap_or(DEFAULT_ADMISSON_STRATEGY), 
            self.eviction_strategy.unwrap_or(DEFAULT_EVICTION_STRATEGY), 
            self.strategy_capacity.unwrap_or(max_entries),
        );

        if let Some(value) = self.admission_threshold {
            strategy_config.threshold = value;
        }

        if let Some(value) = self.admission_width {
            strategy_config.tiny_lfu_width = value;
        }

        if let Some(value) = self.admission_depth {
            strategy_config.tiny_lfu_depth = value;
        }

        if let Some(value) = self.admission_frequency {
            strategy_config.tiny_lfu_frequency = value;
        }

        if let Some(value) = self.window_capacity {
            strategy_config.window_capacity = value;
        }

        let routing_config = RoutingConfig {
            search_partitions: self.search_partitions.unwrap_or(DEFAULT_SEARCH_PARTITIONS)
        };

        let maintenance_config = MaintenanceConfig {
            cooperative: self.cooperative.unwrap_or(DEFAULT_COOPERATIVE),
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
            search: search_config,
            strategy: strategy_config,
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
pub struct RoutingConfig {
    search_partitions: usize
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MaintenanceConfig {
    cooperative: bool,
    hysteresis: f32,
    move_cooldown: usize,
    step_cooldown: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MetricsConfig {
    pub metrics_enabled: bool
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StrategyConfig {
    admission_policy: Admission,
    eviction_policy: Eviction,
    capacity: usize,

    // Admission Parameters
    threshold: f32,
    tiny_lfu_width: usize,
    tiny_lfu_depth: usize,
    tiny_lfu_frequency: u8,
    window_capacity: usize,
}

#[allow(dead_code)]
impl StrategyConfig {
    #[inline]
    pub fn new(admission_policy: Admission, eviction_policy: Eviction, capacity: usize) -> Self {
        Self { 
            admission_policy, 
            eviction_policy, 
            capacity, 
            threshold: 0.0, 
            tiny_lfu_width: 4096, 
            tiny_lfu_depth: 4, 
            tiny_lfu_frequency: 2, 
            window_capacity: 256, 
        }
    }

    #[inline]
    pub fn admission_policy(&self) -> Admission {
        self.admission_policy
    }

    #[inline]
    pub fn eviction_policy(&self) -> Eviction {
        self.eviction_policy
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn validate(&self) -> Result<(), TectonicError> {
        if self.capacity == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Strategy Capacity", 
                issue: "Must be a positive Integer"
            });
        }

        if !self.threshold.is_finite() {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Strategy Threshold", 
                issue: "Must be finite" 
            });
        }

        if self.tiny_lfu_width == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "TinyLFU Width", 
                issue: "Must be a postive integer" 
            });
        }

        if self.tiny_lfu_depth == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "TinyLFU Depth", 
                issue: "Must be a postive integer" 
            });
        }

        if self.tiny_lfu_frequency == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "TinyLFU frequency", 
                issue: "Must be a postive integer" 
            });
        }

        if self.window_capacity == 0 {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Window Capacity", 
                issue: "Must be a postive integer" 
            });
        }

        if self.window_capacity > self.capacity {
            return Err(TectonicError::InvalidParamaterError { 
                param: "Window Capacity", 
                issue: "Must not exceed strategy capacity" 
            });
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub struct  StrategyStructure {
    admission: Box<dyn AdmissionStrategy>,
    eviction: Box<dyn EvictionStrategy>,
}

#[allow(dead_code)]
impl StrategyStructure {
    #[inline]
    pub fn new(admission: Box<dyn AdmissionStrategy>, eviction: Box<dyn EvictionStrategy>) -> Self {
        Self { 
            admission, 
            eviction 
        }
    }

    #[inline]
    pub fn from_config(config: StrategyConfig) -> Result<Self, TectonicError> {
        config.validate()?;

        Ok(Self { 
            admission: build_admission_strategy(&config), 
            eviction: build_eviction_strategy(&config), 
        })
    }

    #[inline]
    pub fn get_admission(&self) -> &dyn AdmissionStrategy {
        self.admission.as_ref()
    }

    #[inline]
    pub fn get_admission_mut(&mut self) -> &mut dyn AdmissionStrategy {
        self.admission.as_mut()
    }

    #[inline]
    pub fn get_eviction(&self) -> &dyn EvictionStrategy {
        self.eviction.as_ref()
    }

    #[inline]
    pub fn get_eviction_mut(&mut self) -> &mut dyn EvictionStrategy {
        self.eviction.as_mut()
    }
}

#[inline]
fn build_admission_strategy(config: &StrategyConfig) -> Box<dyn AdmissionStrategy> {
    match config.admission_policy {
        Admission::Always => {
            Box::new(AlwaysAdmission::new())
        },
        Admission::TwoHit => {
            Box::new(TwoHitAdmission::with_capacity(config.window_capacity))
        },
        Admission::TinyLFU => {
            Box::new(TinyLFUAdmission::with_params(
                config.tiny_lfu_width, 
                config.tiny_lfu_depth, 
                config.tiny_lfu_frequency
            ))
        },
        Admission::WeightedTinyLFU => {
            Box::new(WindowTinyLFUAdmisssion::with_params(
                config.tiny_lfu_width, 
                config.tiny_lfu_depth, 
                config.window_capacity, 
                config.tiny_lfu_frequency,
            ))
        }
    }
}

#[inline]
fn build_eviction_strategy(config: &StrategyConfig) -> Box<dyn EvictionStrategy> {
    match config.eviction_policy {
        Eviction::PartitionedLIFO => {
            Box::new(PartitionedLIFO::with_capacity(config.capacity))
        },
        Eviction::PartitionedFIFO => {
            Box::new(PartitionedFIFO::with_capacity(config.capacity, config.capacity))
        },
        Eviction::PartitionedLRU => {
            Box::new(PartitionedLRU::with_capacity(config.capacity))
        },
        Eviction::SegmentedLRU => {
            Box::new(SegmentedLRU::with_capacity(config.capacity))
        },
        Eviction::VARC => {
            Box::new(VARC::with_capacity(config.capacity))
        }
    }
}

