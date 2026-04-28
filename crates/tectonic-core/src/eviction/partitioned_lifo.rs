// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::{HashMap, hash_map::Entry};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LIFO
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PartitionedLIFO {
    stack: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, NodeValue>
}

impl Default for PartitionedLIFO {
    fn default() -> Self {
        Self { 
            stack: TectonicDoublyLinkedList::default(),
            index_map: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl PartitionedLIFO {
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
impl EvictionStrategy for PartitionedLIFO {
    #[inline]
    fn on_get(&mut self, _entry_id: &UniqueID) {
        // Method is redundant for LIFO functionality.
        self.debug_basic_invariants();
    }

    #[inline]
    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let &value = self.index_map.get(entry_id)?;
        let removed = self.stack.unlink(value)?;
        let map_removed = self.index_map.remove(entry_id);

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(&removed, entry_id);
            debug_assert!(map_removed.is_some());
            self.debug_basic_invariants();
        }

        Some(removed)
    }

    #[inline]
    fn on_insert(&mut self, entry: UniqueID) {
        match self.index_map.entry(entry) {
            Entry::Occupied(_) => {},
            Entry::Vacant(slot) => {
                let value = self.stack.push_back(entry);
                slot.insert(value);

                #[cfg(debug_assertions)]
                self.debug_basic_invariants();
            }
        }
    }

    #[inline]
    fn get_victim(&mut self) -> Option<&UniqueID>{
        #[cfg(debug_assertions)]
        self.debug_basic_invariants();

        self.stack.back()
    }

    #[inline]
    fn evict_victim(&mut self) -> Option<UniqueID> {
        let victim = self.stack.pop_back()?;
        let removed = self.index_map.remove(&victim);

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                removed.is_some(),
                "Victim value existed in the Stack but not in IndexMap"
            );
            self.debug_basic_invariants();
        }

        Some(victim)
    }

    #[inline]
    fn len(&self) -> usize {
        #[cfg(debug_assertions)]
        self.debug_basic_invariants();

        self.stack.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        #[cfg(debug_assertions)]
        self.debug_basic_invariants();

        self.stack.is_empty()
    }
}