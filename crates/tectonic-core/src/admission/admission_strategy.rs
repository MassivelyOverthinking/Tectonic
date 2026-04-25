
// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// ADMISSION STRATEGIES
// ============================================================

use crate::utility::utils::UniqueID;


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Admission {
    Always,
    Threshold,
    TwoHit,
    TinyLFU,
    WeightedTinyLFU,
    SemAware,
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
    fn on_get(&mut self);

    fn on_insert(&mut self);

    fn on_remove(&mut self);
    
    fn should_admit(&mut self, candidate: &AdmissionCandidate) -> bool;
}