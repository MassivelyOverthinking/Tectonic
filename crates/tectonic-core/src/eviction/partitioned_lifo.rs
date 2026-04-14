// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{eviction::{eviction_entry::EvictionEntry, eviction_strategy::EvictionStrategy}, utility::utils::UniqueID};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[derive(Debug, Clone)]
struct PartitionedLIFO {
    stack: Vec<EvictionEntry>,
}

impl PartitionedLIFO {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

impl EvictionStrategy for PartitionedLIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for LIFO functionality.
    }

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<EvictionEntry> {
        let position = self
            .stack
            .iter()
            .position(|entry| entry.entry_id() == entry_id)?;

        Some(self.stack.swap_remove(position))
    }

    fn on_insert(&mut self, entry: EvictionEntry) {
        self.stack.push(entry);
    }

    fn get_victim(&mut self) -> Option<&EvictionEntry>{
        self.stack.last()
    }

    fn evict_victim(&mut self) -> Option<EvictionEntry> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}