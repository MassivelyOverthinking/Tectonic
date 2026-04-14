// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::eviction::{eviction_entry::EvictionEntry, eviction_strategy::EvictionStrategy};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[derive(Debug, Clone)]
struct PartitionedLIFO {
    stack: Vec<EvictionEntry>,
    size: usize,
}

impl EvictionStrategy for PartitionedLIFO {
    fn on_get(&mut self) {
        todo!()
    }

    fn on_remove(&mut self) {
        todo!()
    }

    fn on_insert(&mut self) {
        todo!()
    }

    fn get_victim(&mut self) {
        todo!()
    }

    fn evict_victim(&mut self) {
        todo!()
    }

    fn len(&self) -> usize {
        
    }

    fn is_empty(&self) -> bool {
        
    }
}