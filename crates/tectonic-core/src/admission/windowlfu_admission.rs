// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::HashMap;

use crate::admission::admission_strategy::{AdmissionStrategy};
use crate::utility::structures::CountMinSketch;
use crate::utility::structures::TectonicDoublyLinkedList;
use crate::utility::typings::NodeValue;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: WINDOW TINY-LFU
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
const DEFAULT_WINDOW_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct WindowTinyLFUAdmisssion {
    sketch: CountMinSketch,
    window: TectonicDoublyLinkedList,
    window_index: HashMap<UniqueID, NodeValue>,
    window_capacity: usize,
    frequency: u8,
}

impl Default for WindowTinyLFUAdmisssion {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl WindowTinyLFUAdmisssion {
    #[inline]
    pub fn new() -> Self {
        Self::with_params(
            DEFAULT_SKETCH_WIDTH, 
            DEFAULT_SKETCH_DEPTH, 
            DEFAULT_WINDOW_CAPACITY, 
            DEFAULT_MIN_FREQUENCY,
        )
    }

    #[inline]
    pub fn with_params(width: usize, depth: usize, capacity: usize, frequency: u8) -> Self {
        let window_capacity = capacity.max(1);
        let min_frequency = frequency.max(1);

        let policy = Self {
            sketch: CountMinSketch::new(width, depth),
            window: TectonicDoublyLinkedList::with_capacity(window_capacity),
            window_index: HashMap::with_capacity(window_capacity),
            window_capacity,
            frequency: min_frequency,
        };

        policy
    }

    #[inline]
    pub fn with_sample_size(width: usize, depth: usize, capacity: usize, frequency: u8, sample_size: usize) -> Self {
        let window_capacity = capacity.max(1);
        let min_frequency = frequency.max(1);

        let policy = Self {
            sketch: CountMinSketch::with_sample_size(
                width, 
                depth, 
                sample_size
            ),
            window: TectonicDoublyLinkedList::with_capacity(window_capacity),
            window_index: HashMap::with_capacity(window_capacity),
            window_capacity,
            frequency: min_frequency,
        };

        #[cfg(debug_assertions)]
        policy.debug_assertions_basic();

        policy
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_basic(&self) {
        debug_assert!(
            self.window_capacity > 0,
            "WindowTinyLFU capacity must be represented by a positive integer"
        );

        debug_assert!(
            self.frequency > 0,
            "WindowTinyLFU frequency must be represented by a positive integer"
        );

        debug_assert_eq!(
            self.window.len(),
            self.window_index.len(),
            "WindowTinyLFU window/IndexMap length mismatch"
        );

        debug_assert!(
            self.window.len() <= self.window_capacity,
            "WindowTinyLFU window exceeds defined capacity"
        );

        debug_assert_eq!(
            self.window.is_empty(),
            self.window_index.is_empty(),
            "WindowTinyLFU window/IndexMap empty-state mismatch"
        );

        debug_assert!(
            self.sketch.width() > 0,
            "WindowTinyLFU sketch width must be represented by a positive integer"
        );

        debug_assert!(
            self.sketch.depth() > 0,
            "WindowTinyLFU sketch depth must be represented by a positive integer"
        );

        debug_assert_eq!(
            self.sketch.count(),
            self.sketch.width() * self.sketch.depth(),
            "WindowTinyLFU intenral sketch counter mismatch"
        );

        debug_assert!(
            self.sketch.sample_size() <= self.sketch.sample_size(),
            "WindowTinyLFU internal sketch size exceeds sample size"
        );
    }
}