// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::{HashMap, hash_map::Entry};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LRU
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PartitionedLRU {
    stack: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, NodeValue>
}

impl Default for PartitionedLRU {
    fn default() -> Self {
        Self { 
            stack: TectonicDoublyLinkedList::default(),
            index_map: HashMap::new(),
        }
    }
}

impl PartitionedLRU {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            stack: TectonicDoublyLinkedList::with_capacity(capacity), 
            index_map: HashMap::with_capacity(capacity),
        }
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_basic_invariants(&self) {
        let stack_length = self.stack.len();
        let map_length = self.index_map.len();

        debug_assert_eq!(
            stack_length,
            map_length,
            "Stack/IndexMap length mismtach: Stack = {}, Map = {}",
            stack_length,
            map_length
        );

        debug_assert!(
            self.stack.is_empty() == self.index_map.is_empty(),
            "Stack/IndexMap state mismatch"
        )
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedLRU {
    fn on_get(&mut self, entry_id: &UniqueID) {
        todo!()
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