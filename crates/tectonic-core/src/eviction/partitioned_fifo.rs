// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::{HashMap};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: PARTITIONED FIFO
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PartitionedFIFO {
    // Possibly user "OrderedHashmap" or "IndexMap" for O(1) lookup.
    stack: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, NodeValue>,
}

impl Default for PartitionedFIFO {
    fn default() -> Self {
        Self { 
            stack: TectonicDoublyLinkedList::default(),
            index_map: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl PartitionedFIFO {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(list_capacity: usize, map_capacity: usize) -> Self {
        Self { 
            stack: TectonicDoublyLinkedList::with_capacity(list_capacity), 
            index_map: HashMap::with_capacity(map_capacity), 
        }
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedFIFO {
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for FIFO functionality.
    }

    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let value = self.index_map.remove(entry_id)?;
        let removed = self.stack.unlink(value)?;

        Some(removed)
    }

    fn on_insert(&mut self, entry: UniqueID) {
        if self.index_map.contains_key(&entry) {
            return;
        }

        let value = self.stack.push_back(entry);
        let _ = self.index_map.insert(entry, value);
    }

    fn get_victim(&mut self) -> Option<&UniqueID> {
        let victim = self.stack.front();
        victim
    }

    fn evict_victim(&mut self) -> Option<UniqueID> {
        let victim = self.stack.pop_front()?;
        let _ = self.index_map.remove(&victim);

        Some(victim)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}