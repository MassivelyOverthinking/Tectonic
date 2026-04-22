// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::{HashMap};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: SEGMENTED LRU
// ============================================================

#[derive(Debug, Clone)]
pub enum SegmentType {
    Probationary,
    Protected
}

#[derive(Debug, Clone)]
pub struct EntryLocation {
    segment: SegmentType,
    node: NodeValue,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SegmentedLRU {
    probationary: TectonicDoublyLinkedList,
    protected: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, EntryLocation>,
    probationary_capacity: usize,
    protected_capacity: usize,
}

impl Default for SegmentedLRU {
    #[inline]
    fn default() -> Self {
        Self { 
            probationary: TectonicDoublyLinkedList::default(),
            protected: TectonicDoublyLinkedList::default(),
            index_map: HashMap::new(),
            probationary_capacity: 100,
            protected_capacity: 100
        }
    }
}

impl SegmentedLRU {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            probationary: TectonicDoublyLinkedList::with_capacity(capacity),
            protected: TectonicDoublyLinkedList::with_capacity(capacity), 
            index_map: HashMap::with_capacity(capacity), 
            probationary_capacity: capacity, 
            protected_capacity: capacity 
        }
    }
}