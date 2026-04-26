// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::admission::admission_strategy::{AdmissionStrategy};
use crate::utility::structures::CountMinSketch;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: TINY-LFU
// ============================================================
// Admission Policy for admitting entries based on a probabilistic frequency
// calculated using custom Count-Min Sketch structure.
// Candidate are only permitted entry once their estimated frequency exceeds
// configurable minimum frequency.
// ---
// This admission policy is primarily used for specific scenarios:
// * Provides memory efficient, high-quality entry admission
// * Easily adapt to diverse admission patterns
// ---
// Performance Characteristics:
// * "on_get()" -> O(n)
// * "on_insert()" -> O(1)
// * "on_remove()" -> O(1)
// * "should_admit()" -> O(n)
// * Fix size memory footprint
// * Highly cache efficient and scan resistant structure

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

#[allow(dead_code)]
impl TinyLFUAdmission {
    #[inline]
    pub fn with_params(width: usize, depth: usize, frequency: u8) -> Self {
        let frequency = frequency.max(1);

        let policy = Self {
            sketch: CountMinSketch::new(width, depth),
            frequency,
        };

        #[cfg(debug_assertions)]
        policy.debug_assertions_basic();

        policy
    }

    #[inline]
    pub fn with_sample_size(width: usize, depth: usize, frequency: u8, sample_size: usize) -> Self {
        let frequency = frequency.max(1);

        let policy = Self {
            sketch: CountMinSketch::with_sample_size(width, depth, sample_size),
            frequency
        };

        #[cfg(debug_assertions)]
        policy.debug_assertions_basic();

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

    #[inline]
    pub fn estimated_frequency(&self, entry_id: &UniqueID) -> u8 {
        self.sketch.estimate(entry_id)
    }

    #[inline]
    pub fn record_access(&mut self, entry_id: &UniqueID) {
        self.sketch.increment(entry_id);
    }

    #[inline]
    pub fn frequency(&self) -> u8 {
        self.frequency
    }

    #[inline]
    pub fn clear(&mut self) {
        self.sketch.clear();

        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_basic(&self) {
        debug_assert!(
            self.frequency > 0,
            "TinyLFU frequency variable must be represented by a positive integer"
        );

        debug_assert!(
            self.sketch.width() > 0,
            "TinyLFU width variable must be represented by a postive integer"
        );

        debug_assert!(
            self.sketch.depth() > 0,
            "TinyLFU depth variable must be represented by a postive integer"
        );

        debug_assert_eq!(
            self.sketch.count(),
            self.sketch.width() * self.sketch.depth(),
            "TinyLFU sketch size mismatch"
        );

        debug_assert!(
            self.sketch.sample_size() > 0,
            "TinyLFU sample size variable must be represented by a postive integer"
        );

        debug_assert!(
            self.sketch.size() <= self.sketch.sample_size(),
            "TinyLFU internal size exceeds current sample size"
        );
    }
}

impl AdmissionStrategy for TinyLFUAdmission {
    #[inline]
    fn on_get(&mut self, entry_id: &UniqueID) {
        self.record_access(entry_id);

        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
    }

    fn on_insert(&mut self, _entry_id: &UniqueID) {
        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
    }

    fn on_remove(&mut self, _entry_id: &UniqueID) {
        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
    }

    fn should_admit(&mut self, entry_id: &UniqueID) -> bool {
        self.record_access(entry_id);

        let should_admit = self
            .sketch
            .estimate_least(entry_id, self.frequency);

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                should_admit,
                self.sketch.estimate(entry_id) >= self.frequency,
                "TinyLFU admission decision failed"
            );
            self.debug_assertions_basic();
        }

        should_admit
    }
}