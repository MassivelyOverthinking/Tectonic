// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::default;

use hashbrown::HashMap;

use crate::admission::admission_strategy::AdmissionStrategy;
use crate::utility::structures::TectonicDoublyLinkedList;
use crate::utility::typings::NodeValue;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: TWO-HIT
// ============================================================

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
            }
        }
    }
}