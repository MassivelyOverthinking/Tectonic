// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::{HashMap};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: PARTITIONED LRU
// ============================================================

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PartitionedLRU {
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

#[allow(dead_code)]
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

    #[inline]
    #[cfg(debug_assertions)]
    pub fn debug_assertions_entry_match(&self, entry_id: &UniqueID, value: NodeValue) {
        let node_payload = self
            .stack
            .get(value)
            .expect("IndexMap value must reference a live Node!");

        debug_assert_eq!(
            node_payload,
            entry_id,
            "IndexMap value did not match requested value"
        );
    }
}

#[allow(dead_code)]
impl EvictionStrategy for PartitionedLRU {
    #[inline]
    fn on_get(&mut self, entry_id: &UniqueID) {
        if let Some(&node_value) = self.index_map.get(entry_id) {
            #[cfg(debug_assertions)] 
            {
                self.debug_basic_invariants();
                self.debug_assertions_entry_match(entry_id, node_value);
            }

            let _moved = self.stack.move_to_back(node_value);

            #[cfg(debug_assertions)]
            {
                debug_assert!(
                    self.stack.is_tail(node_value),
                    "On_get method did not promote Node to Tail position"
                );
                self.debug_basic_invariants();
            }
        } else {
            #[cfg(debug_assertions)]
            self.debug_basic_invariants();
        }
    }

    #[inline]
    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let node_value = self.index_map.get(entry_id)?;

        #[cfg(debug_assertions)]
        {
            self.debug_basic_invariants();
            self.debug_assertions_entry_match(entry_id, *node_value);
        }

        let removed_from_stack = self.stack.unlink(*node_value)?;
        let removed_from_map = self.index_map.remove(entry_id);

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                &removed_from_stack,
                entry_id,
                "Removed node's Payload did not match the requested ID"
            );
            debug_assert!(
                removed_from_map.is_some(),
                "Stack unlink was successfull, but removal from IndexMap failed"
            );
            self.debug_basic_invariants();
        }

        Some(removed_from_stack)
    }

    #[inline]
    fn on_insert(&mut self, entry: UniqueID) {
        if let Some(&node_value) = self.index_map.get(&entry) {
            #[cfg(debug_assertions)]
            {
                self.debug_basic_invariants();
                self.debug_assertions_entry_match(&entry, node_value);
            }
            return;
        }

        let node_value = self.stack.push_back(entry);
        let old_value = self.index_map.insert(entry, node_value);

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                old_value.is_none(),
                "on_insert replaced an existing IndexMap entry unexpectedly"
            );

            debug_assert!(
                self.stack.is_tail(node_value),
                "New Node was not correctly inserted onto the LinkedList Tail"
            );

            let inserted_entry = self
                .stack
                .get(node_value)
                .expect("New Node must reference a live Node value");

            let mapped_value = self
                .index_map
                .get(inserted_entry)
                .expect("Inserted entry must exist in IndexMap");

            debug_assert_eq!(
                *mapped_value,
                node_value,
                "IndexMap handle did not match inserted node handle"
            );

            self.debug_basic_invariants();
        }
    }

    #[inline]
    fn get_victim(&mut self) -> Option<&UniqueID> {
        #[cfg(debug_assertions)]
        self.debug_basic_invariants();

        self.stack.front()
    }

    #[inline]
    fn evict_victim(&mut self) -> Option<UniqueID> {
        #[cfg(debug_assertions)]
        self.debug_basic_invariants();

        let victim = self.stack.pop_front()?;
        let removed_from_map = self.index_map.remove(&victim);

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                removed_from_map.is_some(),
                "Removed Node value existed in Stack but not in IndexMap"
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