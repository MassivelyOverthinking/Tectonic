// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::admission::admission_strategy::AdmissionStrategy;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: ALWAYS
// ============================================================
// Admission Policy that admits every incoming candidate.
// ---
// This admission policy is primarily used for specific scenarios:
// * Baseline admission for Benchmarking against other strategies.
// * Simple default strategy
// * When admission-related filtering is not a primary concern.
// ---
// Performance Characteristics:
// * "on_get()" -> O(1)
// * "on_insert()" -> O(1)
// * "on_remove()" -> O(1)
// * "should_admit()" -> O(1)
// * No internal metadata
// * No unecessary Heap allocation

#[derive(Debug, Clone, Copy)]
pub struct AlwaysAdmission;

// ============================================================
// ADMISSION STRATEGY: CONSTRUCTORS
// ============================================================

impl Default for AlwaysAdmission {
    #[inline]
    fn default() -> Self {
        Self
    }
}

#[allow(dead_code)]
impl AlwaysAdmission {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================
    // ADMISSION STRATEGY: DEBUGGING
    // ============================================================ 

    #[inline]
    #[cfg(debug_assertions)]
    pub fn debug_assertions_state(&self) {
        debug_assert_eq!(
            core::mem::size_of::<Self>(),
            0,
            "Always Admission Policy should remain stateless and hold 0 memory size"
        );
    }
}

// ============================================================
// ADMISSION STRATEGY: STRATEGY METHODS
// ============================================================

impl AdmissionStrategy for AlwaysAdmission {
    #[inline]
    fn on_get(&mut self, _entry_id: &UniqueID) {
        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn on_insert(&mut self, _entry_id: &UniqueID) {
        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn on_remove(&mut self, _entry_id: &UniqueID) {
        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn should_admit(&mut self, _candidate: &super::admission_strategy::AdmissionCandidate) -> bool {
        #[cfg(debug_assertions)]
        self.debug_assertions_state();

        true
    }
}