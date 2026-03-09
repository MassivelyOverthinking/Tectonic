// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::time::Instant;

// ============================================================
// INTERNAL VECTOR METRICS
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