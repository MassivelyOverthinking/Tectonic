// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::admission::admission_strategy::{AdmissionStrategy};
use crate::utility::structures::CountMinSketch;
use crate::utility::utils::UniqueID;


// ============================================================
// ADMISSION STRATEGY: TINY-LFU
// ============================================================
// Admission Policy for admitting entries that appear twice.
// Candidate entry will always be permitted access if observed twice.
// ---
// This admission policy is primarily used for specific scenarios:
// * Protect cache from general entry pollution.
// * Simple, predictable and cheap admission strategy.
// ---
// Performance Characteristics:
// * "on_get()" -> O(1)
// * "on_insert()" -> O(1)
// * "on_remove()" -> O(1)
// * "should_admit()" -> O(1)
// * No unecessary Heap allocation
// * Trim history using FIFO structure.

const DEFAULT_SKETCH_WIDTH: usize = 4096;
const DEFAULT_SKETCH_DEPTH: usize = 4;
const DEFAULT_MIN_FREQUENCY: u8 = 2;

#[derive(Debug, Clone)]
pub struct TinyLFUAdmission {
    sketch: CountMinSketch,
    frequency: u8,
}

impl Default for TinyLFUAdmission {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TinyLFUAdmission {
    #[inline]
    pub fn with_params(width: usize, depth: usize, frequency: u8) -> Self {
        let frequency = frequency.max(1);

        let policy = Self {
            sketch: CountMinSketch::new(width, depth),
            frequency,
        };

        policy
    }

    #[inline]
    pub fn new() -> Self {
        Self::with_params(
            DEFAULT_SKETCH_WIDTH, 
            DEFAULT_SKETCH_DEPTH,
            DEFAULT_MIN_FREQUENCY,
        )
    }
}