// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::mem::replace;

use crate::utility::{typings::NodeValue, utils::UniqueID};

// ============================================================
// CUSTOM DATA STRUCTURES
// ============================================================

#[derive(Debug, Clone)]
enum ListSlot {
    Occupied(ListNode),
    Free { next_free: Option<NodeValue>}
}

#[derive(Debug, Clone)]
pub struct ListNode {
    payload: UniqueID,
    previous: Option<NodeValue>,
    next: Option<NodeValue>,
}

impl ListNode {
    #[inline]
    pub fn new(id: UniqueID) -> Self {
        Self { 
            payload: id, 
            previous: None, 
            next: None 
        }
    }

    #[inline]
    pub fn get_payload(&self) -> &UniqueID {
        &self.payload
    }

    #[inline]
    pub fn get_next(&self) -> Option<NodeValue> {
        self.next
    }

    #[inline]
    pub fn get_previous(&self) -> Option<NodeValue> {
        self.previous
    }
}

#[derive(Debug, Clone)]
pub struct TectonicDoublyLinkedList {
    list: Vec<ListSlot>,
    free_head: Option<NodeValue>,
    head: Option<NodeValue>,
    tail: Option<NodeValue>,
    size: usize,
}

impl Default for TectonicDoublyLinkedList {
    #[inline]
    fn default() -> Self {
        Self { 
            list: Vec::new(),
            free_head: None,
            head: None,
            tail: None,
            size: 0 
        }
    }
}

impl TectonicDoublyLinkedList {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            list: Vec::with_capacity(capacity),
            free_head: None,
            head: None,
            tail: None,
            size: 0 
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn get_head(&self) -> Option<NodeValue> {
        self.head
    }

    #[inline]
    pub fn get_tail(&self) -> Option<NodeValue> {
        self.tail
    }

    #[inline]
    pub fn front(&self) -> Option<&UniqueID> {
        let head = self.head?;
        Some(&self.get_node(head)?.payload)
    }

    #[inline]
    pub fn back(&self) -> Option<&UniqueID> {
        let tail = self.tail?;
        Some(&self.get_node(tail)?.payload)
    }

    #[inline]
    pub fn contains_hanlde(&self, value: NodeValue) -> bool {
        self.get_node(value).is_some()
    }

    #[inline]
    pub fn get(&self, value: NodeValue) -> Option<&UniqueID> {
        Some(&self.get_node(value)?.payload)
    }

    #[inline]
    pub fn previous_of(&self, value: NodeValue) -> Option<NodeValue> {
        self.get_node(value)?.previous
    }

    #[inline]
    pub fn next_of(&self, value: NodeValue) -> Option<NodeValue> {
        self.get_node(value)?.next
    }

    // ============================================================
    // INSTERTION METHODS
    // ============================================================

    #[inline]
    pub fn push_front(&mut self, id: UniqueID) -> NodeValue {
        let old_head = self.head;

        let new_node = self.allocate_node(
            ListNode {
                payload: id,
                previous: None,
                next: old_head
            }
        );

        match old_head {
            Some(head_value) => {
                let head_node = self
                    .get_mut_node(head_value)
                    .expect("Head must reference a live node");
                debug_assert!(
                    head_node.previous.is_none(),
                    "Head node unexpectedly has a previous node"
                );
                head_node.previous = Some(new_node);
            },
            None => {
                debug_assert!(
                    self.tail.is_none(), 
                    "Linked List had Head but no Tails"
                );
                self.tail = Some(new_node);
            }
        }

        self.head = Some(new_node);
        self.size += 1;

        new_node
    }

    #[inline]
    pub fn push_back(&mut self, id: UniqueID) -> NodeValue {
        let old_tail = self.tail;

        let new_node = self.allocate_node(
            ListNode {
                payload: id,
                previous: None,
                next: old_tail
            }
        );

        match old_tail {
            Some(tail_value) => {
                let tail_node = self
                    .get_mut_node(tail_value)
                    .expect("Tail must reference a live node");
                debug_assert!(
                    tail_node.previous.is_none(),
                    "Tail node unexpectedly has a previous node"
                );
                tail_node.previous = Some(new_node);
            },
            None => {
                debug_assert!(
                    self.head.is_none(), 
                    "Linked List had Tail but no Head"
                );
                self.head = Some(new_node);
            }
        }

        self.tail = Some(new_node);
        self.size += 1;

        new_node
    }

    // ============================================================
    // NODE RETRIEVAL
    // ============================================================

    #[inline]
    pub fn get_node(&self, value: NodeValue) -> Option<&ListNode> {
        match self.list.get(value)? {
            ListSlot::Occupied(node) => Some(node),
            ListSlot::Free { .. } => None,
        }
    }

    #[inline]
    pub fn get_mut_node(&mut self, value: NodeValue) -> Option<&mut ListNode> {
        match self.list.get_mut(value)? {
            ListSlot::Occupied(node) => Some(node),
            ListSlot::Free { .. } => None,
        }
    }

    // ============================================================
    // NODE STORAGE
    // ============================================================

    #[inline]
    fn allocate_node(&mut self, node: ListNode) -> NodeValue {
        if let Some(free_index) = self.free_head {
            let next_free = match self.list[free_index] {
                ListSlot::Free { next_free } => next_free,
                ListSlot::Occupied(_) => unreachable!("Pointing at occupied Slot!")
            };

            self.head = next_free;
            self.list[free_index] = ListSlot::Occupied(node);
            free_index
        } else {
            let next = self.list.len();
            self.list.push(ListSlot::Occupied(node));
            next
        }
    }

    #[inline]
    fn free_node(&mut self, value: NodeValue) -> Option<ListNode> {
        let old_slot = replace(
            self.list.get_mut(value)?,
            ListSlot::Free { 
                next_free: self.free_head, 
            },
        );

        match old_slot {
            ListSlot::Occupied(node) => {
                self.free_head = Some(value);
                Some(node)
            },
            ListSlot::Free { next_free } => {
                self.list[value] = ListSlot::Free { next_free };
                None
            }
        }
    } 
}
