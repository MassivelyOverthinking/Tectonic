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

#[allow(dead_code)]
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

    #[inline]
    #[cfg(debug_assertions)]
    pub fn debug_assertions_basic_state(&self) {
        let probationary_length = self.probationary.len();
        let protected_length = self.protected.len();
        let map_length = self.index_map.len();
        let total_length = probationary_length + protected_length;

        debug_assert_eq!(
            total_length,
            map_length,
            "SegmentedLRU state mismatch: Probationary length={}, Protected length={}, Total length={}, Map length={}",
            probationary_length,
            protected_length,
            total_length,
            map_length
        );

        debug_assert_eq!(
            self.probationary.is_empty(),
            probationary_length == 0,
            "Probationary empty-state mismatch"
        );

        debug_assert_eq!(
            self.protected.is_empty(),
            protected_length == 0,
            "Protected empty-state mismatch"
        );

        debug_assert!(
            probationary_length <= self.probationary_capacity,
            "Probationary segment exceed capacity: Length={}, Capacity={}",
            probationary_length,
            self.probationary_capacity
        );

        debug_assert!(
            protected_length <= self.protected_capacity,
            "Protected segment exceed capacity: Length={}, Capacity={}",
            protected_length,
            self.protected_capacity
        );
    }

    #[inline]
    #[cfg(debug_assertions)]
    pub fn debug_assertions_capacity_state(&self) {
        debug_assert!(
            self.probationary_capacity > 0,
            "Probationary Segment capacity must exceed 0"
        );

        debug_assert!(
            self.protected_capacity > 0,
            "Protected Segment capacity must exceed 0"
        );
    }
}

#[allow(dead_code)]
impl EvictionStrategy for SegmentedLRU {
    #[inline]
    fn on_get(&mut self, entry_id: &UniqueID) {
        todo!()
    }

    #[inline]
    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        todo!()
    }

    #[inline]
    fn on_insert(&mut self, entry: UniqueID) {
        todo!()
    }

    #[inline]
    fn get_victim(&mut self) -> Option<&UniqueID> {
        todo!()
    }

    #[inline]
    fn evict_victim(&mut self) -> Option<UniqueID> {
        todo!()
    }

    #[inline]
    fn len(&self) -> usize {
        self.probationary.len() + self.protected.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.probationary.is_empty() && self.protected.is_empty()
    }
}