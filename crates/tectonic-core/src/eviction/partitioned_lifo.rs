// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::eviction::eviction_strategy::EvictionStrategy;
use crate::utility::utils::UniqueID;

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PartitionedLIFO {
    stack: Vec<UniqueID>,
}

impl Default for PartitionedLIFO {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedLIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for LIFO functionality.
    }

    fn on_remove(&mut self, _entry_id: &UniqueID) -> Option<UniqueID> {
        todo!()
    }

    fn on_insert(&mut self, entry: UniqueID) {
        self.stack.push(entry);
    }

    fn get_victim(&mut self) -> Option<&UniqueID>{
        self.stack.last()
    }

    fn evict_victim(&mut self) -> Option<UniqueID> {
        self.stack.pop()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}