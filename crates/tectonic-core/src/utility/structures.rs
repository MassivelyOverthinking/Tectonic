// ============================================================
// IMPORTS AND MODULES
// ============================================================

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
    pub fn get_node(&self, value: NodeValue) -> Option<&ListNode> {
        match self.list.get(value)? {
            ListSlot::Occupied(node) => Some(node),
            ListSlot::Free { .. } => None,
        }
    }
}
