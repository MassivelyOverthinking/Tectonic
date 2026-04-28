
// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGIES
// ============================================================

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
    id: UniqueID,
    score: Option<f32>,
    cost: usize,
    priority: u32,
}

#[allow(dead_code)]
pub trait AdmissionStrategy {
    fn on_get(&mut self, _entry_id: &UniqueID);

    fn on_insert(&mut self, _entry_id: &UniqueID);

    fn on_remove(&mut self, _entry_id: &UniqueID);
    
    fn should_admit(&mut self, _entry_id: &UniqueID) -> bool;
}