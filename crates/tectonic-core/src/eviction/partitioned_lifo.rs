// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::HashMap;

use crate::eviction::eviction_strategy::EvictionStrategy;
use crate::utility::utils::UniqueID;

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PartitionedLIFO {
    stack: Vec<UniqueID>,
    positions: HashMap<UniqueID, usize>
}

impl Default for PartitionedLIFO {
    fn default() -> Self {
        Self { 
            stack: Vec::new(),
            positions: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl PartitionedLIFO {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            stack: Vec::with_capacity(capacity),
            positions: HashMap::with_capacity(capacity)
        }
    }

    pub fn contains(&self, entry_id: &UniqueID) -> bool {
        self.positions.contains_key(entry_id)
    }

    pub fn reverse(&mut self, additional: usize) {
        self.stack.reserve(additional);
        self.positions.reserve(additional);
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.positions.clear();
    }

    #[cfg(debug_assertions)]
    fn debug_assertions_consistent(&self) {
        debug_assert_eq!(self.stack.len(), self.positions.len())

        for (index, id) in self.stack.iter().enumerate() {
            let stored_index = self
                .positions
                .get(id)
                .expect("PartitionedLIFO invariant violated: Missing id in position");

            debug_assert_eq!(*stored_index, index, "PartitionedLIFO invariant violated: Incorrect id stored");
        }
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedLIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for LIFO functionality.
    }

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let index = self.positions.remove(entry_id)?;

        let last_index = self.stack.len() - 1;
        self.stack.swap(index, last_index);
        let removed_value = self.stack.pop();

        if index < self.stack.len() {
            let swapped_id = self.stack[index];
            self.positions.insert(swapped_id, index);
        };

        #[cfg(debug_assertions)]
        self.debug_assertions_consistent();

        removed_value
    }

    fn on_insert(&mut self, entry: UniqueID) {
        let index = self.stack.len();
        self.stack.push(entry);
        self.positions.insert(entry, index);

        #[cfg(debug_assertions)]
        self.debug_assertions_consistent();
    }

    fn get_victim(&mut self) -> Option<&UniqueID>{
        self.stack.last()
    }

    fn evict_victim(&mut self) -> Option<UniqueID> {
        let victim = self.stack.pop()?;
        self.positions.remove(&victim);

        #[cfg(debug_assertions)]
        self.debug_assertions_consistent();

        Some(victim)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}