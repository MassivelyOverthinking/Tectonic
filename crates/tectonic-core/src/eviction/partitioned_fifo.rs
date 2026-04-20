// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::VecDeque;

use crate::{eviction::{eviction_strategy::EvictionStrategy}, utility::utils::UniqueID};

// ============================================================
// EVICTION STRATEGY: PARTITIONED FIFO
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PartitionedFIFO {
    // Possibly user "OrderedHashmap" or "IndexMap" for O(1) lookup.
    stack: VecDeque<UniqueID>,
}

impl Default for PartitionedFIFO {
    fn default() -> Self {
        Self { stack: VecDeque::new() }
    }
}

impl PartitionedFIFO {
    pub fn new(&self) -> Self {
        Self::default()
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_invariant(&self) {
        use std::collections::HashSet;

        let mut elements = HashSet::with_capacity(self.stack.len());
        for value in &self.stack {
            let inserted = elements.insert(value);
            debug_assert!(inserted, "PartitionedFIFO invariant error: Duplicate entry: {:?}", value)
        }
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedFIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for FIFO functionality.
        self.debug_assertions_invariant();
    }

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let result = if let Some(position) = self.stack.iter().position(|id| id == entry_id) {
            Some(
                self.stack
                    .remove(position)
                    .expect("Position found by removal failed"),
            )
        } else {
            None
        };

        self.debug_assertions_invariant();
        result
    }

    fn on_insert(&mut self, entry: UniqueID) {
        self.stack.push_back(entry);

        self.debug_assertions_invariant();
    }

    fn get_victim(&mut self) -> Option<&UniqueID> {
        let result = self.stack.front();

        result
    }

    fn evict_victim(&mut self) -> Option<UniqueID> {
        let result = self.stack.pop_front();

        self.debug_assertions_invariant();
        result
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}