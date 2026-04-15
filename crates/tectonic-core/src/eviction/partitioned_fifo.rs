// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{eviction::{eviction_strategy::EvictionStrategy}, utility::utils::UniqueID};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[derive(Debug, Clone)]
struct PartitionedFIFO {
    // Possibly user "OrderedHashmap" or "IndexMap" for O(1) lookup.
    stack: Vec<UniqueID>,
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

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        todo!()
    }

    fn on_insert(&mut self, entry: UniqueID) {
        todo!()
    }

    fn get_victim(&mut self) -> Option<&UniqueID> {
        todo!()
    }

    fn evict_victim(&mut self) -> Option<UniqueID> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}