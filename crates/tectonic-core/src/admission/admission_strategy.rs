
// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGIES
// ============================================================
// Fundamental interface and Enum values functioning as the base for dynamic Admission Policy implementations.
// Used to determine whether an incoming Vector entry provides enough value to be included in cache.
// ---
// Trait methods:
// on_get()         -> Update Admission structure on Cache .get() method calls.
// on_insert()      -> Update Admission structure on Cache .insert() method calls.
// on_remove()      -> Update Admission structure on Cache .remove() method calls.
// should_admit()   -> Confirm whether an incoming value should be admitted into cache-instance.

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Admission {
    Always,
    // Threshold (DEPRECATED),
    TwoHit,
    TinyLFU,
    WeightedTinyLFU,
    // SemAware (DEPRECATED),
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct AdmissionCandidate {
    // Internal struct for Admission Candidancy.
    // Currently not utilised (SemAware).
    id: UniqueID,           // Generational ID.
    score: Option<f32>,     // Semantic Score (SemAware).
    cost: usize,            // Admission cost. 
    priority: u32,          // Entry priority. 
}

#[allow(dead_code)]
pub trait AdmissionStrategy {
    // Flexible Rust interface.
    fn on_get(&mut self, _entry_id: &UniqueID);

    fn on_insert(&mut self, _entry_id: &UniqueID);

    fn on_remove(&mut self, _entry_id: &UniqueID);
    
    fn should_admit(&mut self, _entry_id: &UniqueID) -> bool;
}