
// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// EVICTION STRATEGIES
// ============================================================

use crate::{eviction::eviction_entry::EvictionEntry, utility::utils::UniqueID};

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
    fn on_get(&mut self, entry_id: &UniqueID);

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<EvictionEntry>;

    fn on_insert(&mut self, entry: EvictionEntry);

    fn get_victim(&mut self) -> Option<&EvictionEntry>;

    fn evict_victim(&mut self) -> Option<EvictionEntry>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}