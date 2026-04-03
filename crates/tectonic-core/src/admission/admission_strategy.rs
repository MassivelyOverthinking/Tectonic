
// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// ADMISSION STRATEGIES
// ============================================================


#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Admission {
    Always,
    Threshold,
    Priority,
    TinyLRU,
    Weighted,
    SemAware,
}

#[allow(dead_code)]
pub trait AdmissionStrategy {
    fn on_get(&mut self);

    fn on_insert(&mut self);

    fn on_remove(&mut self);
    
    fn should_admit(&mut self) -> bool;
}