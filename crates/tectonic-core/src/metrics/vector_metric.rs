// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::time::Instant;

// ============================================================
// INTERNAL CACHE METRICS
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VectorMetrics<const D: usize> {
    created_at: Instant,
    last_accessed: Instant,
    dimensions: usize,
    times_accessed: u32, 
}

impl<const D: usize> VectorMetrics<D> {
    pub fn default() -> Self {
        Self { 
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            dimensions: D,
            times_accessed: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CacheMetrics {
    created_at: Instant,
    size: usize,
    load_factor: f32,
    actions: ActionMetrics,
    latency: LatencyMetrics,
}

impl CacheMetrics {
    pub fn default() -> Self {
        Self { 
            created_at: Instant::now(),
            size: 0, 
            load_factor: 0.0, 
            actions: ActionMetrics::default(), 
            latency: LatencyMetrics::default(), 
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ActionMetrics {
    total_actions: usize,
    insert_actions: usize,
    remove_actions: usize,
    get_actions: usize,
}

impl ActionMetrics {
    pub fn default() -> Self {
        Self { 
            total_actions: 0,
            insert_actions: 0,
            remove_actions: 0,
            get_actions: 0, 
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct LatencyMetrics {
    total_latency: f32,
    average_latency: f32,
    highest_latency: f32,
    lowest_latency: f32,
}

impl LatencyMetrics {
    pub fn default() -> Self {
        Self { 
            total_latency: 0.0,
            average_latency: 0.0,
            highest_latency: 0.0,
            lowest_latency: 0.0,
        }
    }
}