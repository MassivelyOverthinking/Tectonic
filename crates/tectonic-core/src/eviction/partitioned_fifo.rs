// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{eviction::{eviction_entry::EvictionEntry, eviction_strategy::EvictionStrategy}, utility::utils::UniqueID};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[derive(Debug, Clone)]
struct PartitionedFIFO {
    stack: Vec<EvictionEntry>,
}

impl Default for PartitionedFIFO {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

impl EvictionStrategy for PartitionedFIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for FIFO functionality.
    }

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<EvictionEntry> {
        todo!()
    }

    fn on_insert(&mut self, entry: EvictionEntry) {
        todo!()
    }

    fn get_victim(&mut self) -> Option<&EvictionEntry> {
        todo!()
    }

    fn evict_victim(&mut self) -> Option<EvictionEntry> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}