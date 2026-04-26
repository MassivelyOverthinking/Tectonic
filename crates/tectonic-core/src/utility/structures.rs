// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{hash::{DefaultHasher, Hash, Hasher}, mem::replace};

use hashbrown::DefaultHasher;

use crate::utility::{typings::NodeValue, utils::UniqueID};

// ============================================================
// CUSTOM DATA STRUCTURES
// ============================================================

// ============================================================
// DATA STRUCTURE: COUNT-MIN SKETCH
// ============================================================

#[derive(Debug, Clone)]
pub struct CountMinSketch {
    counters: Vec<u8>,
    width: usize,
    depth: usize,
    mask: usize,
    sample_size: usize,
    size: usize,
}

impl CountMinSketch {
    #[inline]
    pub fn new(width: usize, depth: usize) -> Self {
        let width = width.max(1).next_power_of_two();
        let depth = depth.max(1);

        let counter_count = width
            .checked_mul(depth)
            .expect("CountMinSketch dimensions overflow");

        Self { 
            counters: vec![0; counter_count], 
            width, 
            depth, 
            mask: width - 1, 
            sample_size: counter_count.max(1), 
            size: 0 
        }
    }
}

// ============================================================
// COUNT-MIN SKETCH => HASH HELPER
// ============================================================

#[inline]
fn hash_item<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}


#[inline]
fn split_mix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

// ============================================================
// DATA STRUCTURE: DOUBLY LINKED LIST
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    #[cfg(debug_assertions)]
    fn debug_assertions_shallow(&self) {
        let size = self.size;
        let head = self.head;
        let tail = self.tail;

        debug_assert_eq!(
            size == 0,
            head.is_none() && tail.is_none(),
            "Empty-state mismatch: Size={}, Head={:?}, Tail={:?}",
            size,
            head,
            tail
        );

        if size == 1 {
            debug_assert_eq!(
                head,
                tail,
                "Single-element invariance: Head and Tail are not identical"
            );
        }

        if let Some(head_value) = head {
            let head_node = self
                .get_node(head_value)
                .expect("Head must be a valid, live Node");
            debug_assert!(
                head_node.previous.is_none(),
                "Head node must not contain Previous value"
            );
        }

        if let Some(tail_value) = tail {
            let tail_node = self
                .get_node(tail_value)
                .expect("Tail must be a valid, live Node");
            debug_assert!(
                tail_node.next.is_none(),
                "Tail node must not contain Next value"
            );
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
    pub fn contains_handle(&self, value: NodeValue) -> bool {
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

    #[inline]
    pub fn is_head(&self, value: NodeValue) -> bool {
        #[cfg(debug_assertions)]
        self.debug_assertions_shallow();

        self.head == Some(value)
    }

    #[inline]
    pub fn is_tail(&self, value: NodeValue) -> bool {
        #[cfg(debug_assertions)]
        self.debug_assertions_shallow();

        self.tail == Some(value)
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
                previous: old_tail,
                next: None
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
    // REMOVAL METHODS
    // ============================================================

    #[inline]
    pub fn pop_front(&mut self) -> Option<UniqueID> {
        let head = self.head?;
        let removed = self.unlink(head)?;
        Some(removed)
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<UniqueID> {
        let tail = self.tail?;
        let removed = self.unlink(tail)?;
        Some(removed)
    }

    #[inline]
    pub fn unlink(&mut self, value: NodeValue) -> Option<UniqueID> {
        self.detach(value)?;

        let removed_node = self.free_node(value)?;
        debug_assert!(self.size > 0, "Linked List size inconsistency");
        self.size -= 1;

        #[cfg(debug_assertions)]
        self.debug_assertions_shallow();

        Some(removed_node.payload)
    }

    // ============================================================
    // MOVE METHODS
    // ============================================================

    #[inline]
    pub fn detach(&mut self, value: NodeValue) -> Option<()> {
        let (previous, next) = {
            let node = self.get_node(value)?;
            (node.previous, node.next)
        };

        self.detach_from_previous(previous, next, value);
        self.detach_from_next(previous, next, value);

        let node = self.get_mut_node(value)?;
        node.previous = None;
        node.next = None;

        #[cfg(debug_assertions)]
        {
            self.debug_assertions_shallow();
        };

        Some(())
    }

    #[inline]
    pub fn attach_as_tail(&mut self, value: NodeValue) -> Option<()> {
       let old_tail = self.tail;

       {
            let node = self.get_mut_node(value)?;
            debug_assert!(
                node.previous.is_none() && node.next.is_none(),
                "Method requires a detached Node, but none where found!"
            );
            node.previous = old_tail;
            node.next = None;
       };

       match old_tail {
           Some(tail_value) => {
                let tail_node = self
                    .get_mut_node(tail_value)
                    .expect("Tail must reference a live Node");
                debug_assert!(
                    tail_node.next.is_none(),
                    "Old Tail node unexpectedly had a Next value"
                );
                tail_node.next = Some(value);
           },
           None => {
                debug_assert!(
                    self.head.is_none(),
                    "LinkedList has no Tail value, but still possesses a Head"
                );
                self.head = Some(value);
           }
       }

       self.tail = Some(value);

       #[cfg(debug_assertions)]
       self.debug_assertions_shallow();

       Some(())
    }

    #[inline]
    pub fn attach_as_head(&mut self, value: NodeValue) -> Option<()> {
       let old_head = self.head;

       {
            let node = self.get_mut_node(value)?;
            debug_assert!(
                node.previous.is_none() && node.next.is_none(),
                "Method requires a detached Node, but none where found!"
            );
            node.previous = old_head;
            node.next = None;
       };

       match old_head {
           Some(head_value) => {
                let head_node = self
                    .get_mut_node(head_value)
                    .expect("Head must reference a live Node");
                debug_assert!(
                    head_node.previous.is_none(),
                    "Old Head node unexpectedly had a Previous value"
                );
                head_node.previous = Some(value);
           },
           None => {
                debug_assert!(
                    self.tail.is_none(),
                    "LinkedList has no Head value, but still possesses a Tails"
                );
                self.tail = Some(value);
           }
       }

       self.head = Some(value);

       #[cfg(debug_assertions)]
       self.debug_assertions_shallow();

       Some(())
    }

    #[inline]
    pub fn move_to_back(&mut self, value: NodeValue) -> Option<()> {
        if self.is_tail(value) {
            #[cfg(debug_assertions)]
            self.debug_assertions_shallow();
            return Some(());
        }

        self.detach(value)?;
        self.attach_as_tail(value)?;

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                self.is_tail(value),
                "Move_to_back condition failed: Requested Node was not the Tail"
            );
            self.debug_assertions_shallow();
        };

        Some(())
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

            self.free_head = next_free;
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

    // ============================================================
    // HELPER METHODS
    // ============================================================

    #[inline]
    pub fn detach_from_previous(&mut self, previous: Option<usize>, next: Option<usize>, value: NodeValue) {
        match previous {
            Some(previous_value) => {
                let previous_node = self
                    .get_mut_node(previous_value)
                    .expect("Previous value must reference a live node");
                debug_assert_eq!(
                    previous_node.next,
                    Some(value),
                    "Previous node did not point to target value"
                );
                previous_node.next = next;
            },
            None => {
                debug_assert_eq!(
                    self.tail,
                    Some(value),
                    "Node with no next reference was not Tail value"
                );
                self.tail = previous;
            }
        }
    }

    #[inline]
    pub fn detach_from_next(&mut self, previous: Option<usize>, next: Option<usize>, value: NodeValue) {
        match next {
            Some(next_value) => {
                let next_node = self
                    .get_mut_node(next_value)
                    .expect("Next value must reference a live node");
                debug_assert_eq!(
                    next_node.previous,
                    Some(value),
                    "Next node did not point to target value"
                );
                next_node.previous = previous;
            },
            None => {
                debug_assert_eq!(
                    self.head,
                    Some(value),
                    "Node with no next reference was not Head value"
                );
                self.head = next;
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.list.clear();
        self.free_head = None;
        self.head = None;
        self.tail = None;
        self.size = 0;
    }
}
