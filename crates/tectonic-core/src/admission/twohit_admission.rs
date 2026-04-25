// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::HashMap;

use crate::admission::admission_strategy::AdmissionStrategy;
use crate::utility::structures::TectonicDoublyLinkedList;
use crate::utility::typings::NodeValue;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: TWO-HIT
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

#[derive(Debug, Clone)]
pub struct TwoHitAdmission {
    history: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, NodeValue>,
    capacity: usize,
}

// ============================================================
// ADMISSION STRATEGY: CONSTRUCTORS
// ============================================================

impl Default for TwoHitAdmission {
    #[inline]
    fn default() -> Self {
        Self::with_capacity(1024)
    }
}

impl TwoHitAdmission {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            history: TectonicDoublyLinkedList::with_capacity(capacity), 
            index_map: HashMap::with_capacity(capacity), 
            capacity 
        }
    }

    // ============================================================
    // ADMISSION STRATEGY: HELPER METHODS
    // ============================================================

    #[inline]
    fn remove_from_history(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let node = self.index_map.remove(entry_id)?;
        let removed_value = self.history.unlink(node)?;

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                &removed_value,
                entry_id,
                "Two-Hit Admission removed entry from history, but did not match requested ID"
            );
            self.debug_assertions_basic();
        }

        Some(removed_value)
    }

    #[inline]
    fn trim_history(&mut self) {
        while self.history.len() > self.capacity {
            let removed_value = self
                .history
                .pop_front()
                .expect("Popping front failed when attempting to trim history");

            let removed_from_map = self.index_map.remove(&removed_value);

            #[cfg(debug_assertions)]
            {
                debug_assert!(
                    removed_from_map.is_some(),
                    "Removal from interal List succeeded, but IndexMap removal failed"
                );
                self.debug_assertions_basic();
            }
        }
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_basic(&self) {
        debug_assert_eq!(
            self.history.len(),
            self.index_map.len(),
            "Two-Hit Admission mismatch: List / IndexMap length does not match"
        );

        debug_assert!(
            self.history.len() <= self.capacity,
            "Two-Hit Admission mismatch: Internal list length exceeded capacity"
        );

        debug_assert_eq!(
            self.history.is_empty(),
            self.index_map.is_empty(),
            "Two-Hit Admission mismatch: List / IndexMap emptiness-state does not match"
        );
    }
}

impl AdmissionStrategy for TwoHitAdmission {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        todo!()
    }

    fn on_insert(&mut self, _entry_id: &UniqueID) {
        todo!()
    }

    fn on_remove(&mut self, _entry_id: &UniqueID) {
        todo!()
    }

    fn should_admit(&mut self, candidate: &super::admission_strategy::AdmissionCandidate) -> bool {
        todo!()
    }
}