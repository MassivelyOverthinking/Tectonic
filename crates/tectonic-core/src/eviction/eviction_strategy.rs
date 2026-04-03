
// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// EVICTION STRATEGIES
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Eviction {
    PartitionedLIFO,        // Simple partition-related LIFO eviction strategy.
    PartitionedFIFO,        // Simple partition-related FIFO eviction strategy.
    PartitionedLRU,         // Simple partition-related LRU eviction strategy.
    SegmentedLRU,           // A segmented LRU strategy.
    VARC,                   // A Vector-aware and partition-related ARC strategy.
    SemAware,               // Semantically-aware eviction policy.
}

#[allow(dead_code)]
pub trait EvictionStrategy {
    fn on_get(&mut self);

    fn on_remove(&mut self);

    fn on_insert(&mut self);

    fn get_victim(&mut self);

    fn evict_victim(&mut self);
}