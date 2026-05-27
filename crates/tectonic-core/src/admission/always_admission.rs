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

// Default object constructor (Internal use):
impl Default for AlwaysAdmission {
    #[inline]
    fn default() -> Self {
        Self
    }
}

// Widely-used object constructor (External use).
#[allow(dead_code)]
impl AlwaysAdmission {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================
    // ADMISSION STRATEGY: DEBUGGING
    // ============================================================ 
    // Flexible Rust debugging assertions (methods) for ensuring internal Cache consistency, 
    // method outputs vs. inputs, and state handling. 

    #[inline]
    #[cfg(debug_assertions)]
    pub fn debug_assertions_state(&self) {
        // Checks internal memory size of Admission structure to ensure state consistency.
        // Must always be 0, as Always Admission structure stores no concrete values.
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
// Internal Strategy Methods for handling candidate admission across various critical Cache method calls.

impl AdmissionStrategy for AlwaysAdmission {
    #[inline]
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Internal Strategy method executed on .get() functionality from Main Cache.
        // Performs no additional actions - Other than internal state check.

        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn on_insert(&mut self, _entry_id: &UniqueID) {
        // Internal Strategy method executed on .insert() functionality from Main Cache.
        // Performs no additional actions - Other than internal state check.

        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn on_remove(&mut self, _entry_id: &UniqueID) {
        // Internal Strategy method executed on .remove() functionality from Main Cache.
        // Performs no additional actions - Other than internal state check. 

        #[cfg(debug_assertions)]
        self.debug_assertions_state();
    }

    fn should_admit(&mut self, _entry_id: &UniqueID) -> bool {
        // Internal Strategy method executed on .get() functionality from Main Cache.
        // Performs no additional actions - Other than internal state check.
        // Will always return 'True', as all entries are permitted access.

        #[cfg(debug_assertions)]
        self.debug_assertions_state();

        true
    }
}

// ============================================================
// ALWAYS ADMISSION: UNIT TEST
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use core::mem::{size_of, size_of_val};

    // ============================================================
    // ALWAYS ADMISSION: TEST HELPER
    // ============================================================
    // Create UniqueID instance for testing purposes

    fn make_id(gen: u32, id: usize) -> UniqueID {
        UniqueID { 
            slot_id: id, 
            gen_id: gen 
        }
    }

    // ============================================================
    // ALWAYS ADMISSION: CONSTRUCTOR TESTS
    // ============================================================

    #[test]
    fn default_constructor_stateless_behaviour() {
        let policy = AlwaysAdmission::default();

        assert_eq!(
            size_of_val(&policy),
            0,
            "AlwaysAdmission policy should remain zero-sized at runtime",
        );
    }

    #[test]
    fn new_constructor_stateless_behaviour() {
        let policy = AlwaysAdmission::new();

        assert_eq!(
            size_of_val(&policy),
            0,
            "AlwaysAdmission policy should remain zero-sized at runtime",
        );
    }

    #[test]
    fn zero_size_check() {
        assert_eq!(
            size_of::<AlwaysAdmission>(),
            0,
            "AlwaysAdmission policy must remain zero-sized and stateless at runtime"
        )
    }

    #[test]
    fn always_admission_copy_and_clone() {
        fn assert_clone<T: Clone>() {}
        fn assert_copy<T: Copy>() {}

        assert_clone::<AlwaysAdmission>();
        assert_copy::<AlwaysAdmission>();

        let test_policy = AlwaysAdmission::new();
        let test_copy = test_policy;
        let test_clone = test_policy.clone();

        assert_eq!(size_of_val(&test_policy), 0);
        assert_eq!(size_of_val(&test_copy), 0);
        assert_eq!(size_of_val(&test_clone), 0);
    }

    // ============================================================
    // ALWAYS ADMISSION: ADMISSIONN BEHAVIOUR
    // ============================================================

    #[test]
    fn should_admit_must_always_return_true_on_insert() {
        let mut test_policy = AlwaysAdmission::new();
        let test_id = make_id(1, 1);

        assert!(
            test_policy.should_admit(&test_id),
            "AlwaysAdmission instance must admit every candidate value"
        );
    }

    #[test]
    fn should_admit_must_always_return_true_on_insert_multiple() {
        let mut test_policy = AlwaysAdmission::new();
        
        for id in 0..512 as usize {
            let gen_id = id as u32;
            let test_entry = make_id(
                gen_id, 
                id
            );

            assert!(
                test_policy.should_admit(&test_entry),
                "AlwaysAdmission instance must admit every candidate value"
            );
        };
    }

    #[test]
    fn should_admit_remains_true_after_inserts_and_removal() {
        let mut test_policy = AlwaysAdmission::new();

        let test_entry_a = make_id(1, 1);
        let test_entry_b = make_id(2, 2);
        let test_entry_c = make_id(3, 3);

        test_policy.on_get(&test_entry_a);
        test_policy.on_insert(&test_entry_b);
        test_policy.on_remove(&test_entry_c);

        assert!(
            test_policy.should_admit(&test_entry_a),
            "AlwaysAdmission should remain 'True' after 'on_get'"
        );

        assert!(
            test_policy.should_admit(&test_entry_b),
            "AlwaysAdmission should remain 'True' after 'on_insert'"
        );

        assert!(
            test_policy.should_admit(&test_entry_c),
            "AlwaysAdmission should remain 'True' after 'on_remove'"
        );
    }

    #[test]
    fn policy_lifecycle_events_dont_change_behaviour() {
        let mut test_policy = AlwaysAdmission::new();
        let test_id = make_id(1, 1);

        for _ in 0..512 {
            test_policy.on_get(&test_id);
            test_policy.on_insert(&test_id);
            test_policy.on_remove(&test_id);

            assert!(
                test_policy.should_admit(&test_id),
                "AlwaysAdmission behaviour should remain consistent after extended lifecycle"
            );
        };
    }

    #[test]
    fn policy_lifecycle_events_dont_change_memory_size() {
        let mut test_policy = AlwaysAdmission::new();
        let test_id = make_id(1, 1);

        assert_eq!(size_of_val(&test_policy), 0);

        test_policy.on_get(&test_id);
        assert_eq!(
            size_of_val(&test_policy),
            0,
            "'on_get' must not introduce state and increase memory size"
        );

        test_policy.on_insert(&test_id);
        assert_eq!(
            size_of_val(&test_policy),
            0,
            "'on_insert' must not introduce state and increase memory size"
        );

        test_policy.on_remove(&test_id);
        assert_eq!(
            size_of_val(&test_policy),
            0,
            "'on_remove' must not introduce state and increase memory size"
        );

        let _ = test_policy.should_admit(&test_id);
        assert_eq!(
            size_of_val(&test_policy),
            0,
            "'should_admit' must not introduce state and increase memory size"
        );
    }

}