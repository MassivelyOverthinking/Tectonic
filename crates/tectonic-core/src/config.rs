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

        self.strategy.validate()?;
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

    pub fn distance_metric(mut self, value: DistanceMetric) -> Self {
        self.distance_metric = Some(value);
        self
    }

    pub fn admission_strategy(mut self, value: Admission) -> Self {
        self.admission_strategy = Some(value);
        self
    }

    pub fn eviction_strategy(mut self, value: Eviction) -> Self {
        self.eviction_strategy = Some(value);
        self
    }

    pub fn strategy_capacity(mut self, value: usize) -> Self {
        self.strategy_capacity = Some(value);
        self
    }

    pub fn admission_threshold(mut self, value: f32) -> Self {
        self.admission_threshold = Some(value);
        self
    }

    pub fn admission_width(mut self, value: usize) -> Self {
        self.admission_depth = Some(value);
        self
    }

    pub fn admission_depth(mut self, value: usize) -> Self {
        self.admission_depth = Some(value);
        self
    }

    pub fn admission_frequency(mut self, value: u8) -> Self {
        self.admission_frequency = Some(value);
        self
    }

    pub fn window_capacity(mut self, value: usize) -> Self {
        self.window_capacity = Some(value);
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
        const DEFAULT_EVICTION: Eviction = Eviction::PartitionedLRU;
        const DEFAULT_SEARCH_PARTITIONS: usize = 3;
        const DEFAULT_COOPOERATIVE: bool = true;
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

        let strategy_config = StrategyConfig::new(
            self.admission_strategy.unwrap_or(DEFAULT_ADMISSON_STRATEGY), 
            self.eviction_strategy.unwrap_or(DEFAULT_EVICTION_STRATEGY), 
            self.strategy_capacity.unwrap_or(max_entries),
        );

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

        Ok(())
    }

    #[inline]
    fn debug_assertions_state(&self) {
        debug_assert!(self.capacity > 0, "Strategy Config capacity must exceed 0");
        debug_assert!(self.threshold.is_finite(), "Strategy Config threshold must be initie");
        debug_assert!(self.tiny_lfu_width > 0, "Strategy Config widtd must exceed 0");
        debug_assert!(self.tiny_lfu_depth > 0, "Strategy Config depth must exceed 0");
        debug_assert!(self.tiny_lfu_frequency > 0, "Strategy Config frequncy must exceed 0");
        debug_assert!(self.window_capacity > 0, "Strategy Config window capacity must exceed 0");
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
    pub fn from_config(config: StrategyConfig) -> Self {
        #[cfg(debug_assertions)]
        config.debug_assertions_state();

        Self { 
            admission: build_admission_strategy(&config), 
            eviction: build_eviction_strategy(&config), 
        }
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

