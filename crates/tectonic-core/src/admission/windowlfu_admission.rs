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
    fn remove_from_window(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let node = self.window_index.remove(entry_id)?;
        let removed_value = self.window.unlink(node)?;

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                &removed_value,
                entry_id,
                "WindowTinyLFU removed window entry did not match requested ID"
            );
            self.debug_assertions_basic();
        }

        Some(removed_value)
    }

    #[inline]
    fn remember_in_window(&mut self, entry_id: &UniqueID) {
        if let Some(&node) = self.window_index.get(entry_id) {
            let moved_value = self.window.move_to_back(node);

            #[cfg(debug_assertions)]
            {
                debug_assert!(
                    moved_value.is_some(),
                    "WindowTinyLFU failed to refresg existing node value"
                );
                debug_assert!(
                    self.window.is_tail(node),
                    "WindowTinyLFU failed to move entry ID to the correct Tail position"
                );
                self.debug_assertions_basic();
            }
            return;
        }

        let node = self.window.push_back(*entry_id);
        let old_value = self.window_index.insert(*entry_id, node);

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                old_value.is_none(),
                "WindowTinyLFU unexpectedly replaced existing node value"
            );
            debug_assert!(
                self.window.is_tail(node),
                "WindowTinyLFU failed to move entry ID to the correct Tail position"
            );
        }
        self.trim_window();
    }

    #[inline]
    fn trim_window(&mut self) {
        while self.window.len() > self.window_capacity {
            let removed_value = self
                .window
                .pop_front()
                .expect("Window exceeded capacity, but popping values failed");

            let removed_from_map = self.window_index.remove(&removed_value);

            #[cfg(debug_assertions)]
            {
                debug_assert!(
                    removed_from_map.is_some(),
                    "WindowTinyLFU removed value from Window, but not from IndexMap"
                );
            }
        }

        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
    }

    #[inline]
    pub fn estimate_frequency(&self, entry_id: &UniqueID) -> u8 {
        self.sketch.estimate(entry_id)
    }

    #[inline]
    pub fn record_access(&mut self, entry_id: &UniqueID) {
        self.sketch.increment(entry_id);
    }

    #[inline]
    pub fn window_capacity(&self) -> usize {
        self.window_capacity
    }

    #[inline]
    pub fn frequency(&self) -> u8 {
        self.frequency
    }

    #[inline]
    pub fn clear(&mut self) {
        self.sketch.clear();
        self.window.clear();
        self.window_index.clear();

        #[cfg(debug_assertions)]
        self.debug_assertions_basic();
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