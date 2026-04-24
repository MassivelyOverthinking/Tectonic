// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::{HashMap};

use crate::{eviction::eviction_strategy::EvictionStrategy, utility::{structures::TectonicDoublyLinkedList, typings::NodeValue, utils::UniqueID}};

// ============================================================
// EVICTION STRATEGY: VARC
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcType {
    T1,
    T2,
    B1,
    B2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcMissStatus {
    Cold,
    B1Hit,
    B2Hit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArcLocation {
    list: ArcType,
    node: NodeValue,
}

#[derive(Debug, Clone)]
pub struct VARC {
    t1: TectonicDoublyLinkedList,
    t2: TectonicDoublyLinkedList,
    b1: TectonicDoublyLinkedList,
    b2: TectonicDoublyLinkedList,
    index_map: HashMap<UniqueID, ArcLocation>,
    capacity: usize,
    pivot: usize,
}

impl Default for VARC {
    #[inline]
    fn default() -> Self {
        Self::with_capacity(100)
    }
}

impl VARC {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            t1: TectonicDoublyLinkedList::with_capacity(capacity),
            t2: TectonicDoublyLinkedList::with_capacity(capacity), 
            b1: TectonicDoublyLinkedList::with_capacity(capacity), 
            b2: TectonicDoublyLinkedList::with_capacity(capacity), 
            index_map: HashMap::with_capacity(capacity.saturating_mul(2)), 
            capacity, 
            pivot: 0 
        }
    }

    #[inline]
    pub fn resident_len(&self) -> usize {
        self.t1.len() + self.t2.len()
    }

    #[inline]
    pub fn ghost_len(&self) -> usize {
        self.b1.len() + self.b2.len()
    }

    #[inline]
    pub fn list_len(&self) -> usize {
        self.resident_len() + self.ghost_len()
    }

    #[inline]
    pub fn list(&self, list_type: ArcType) -> &TectonicDoublyLinkedList {
        match list_type {
            ArcType::T1 => &self.t1,
            ArcType::T2 => &self.t2,
            ArcType::B1 => &self.b1,
            ArcType::B2 => &self.b2,
        }
    }

    #[inline]
    pub fn list_mut(&mut self, list_type: ArcType) -> &mut TectonicDoublyLinkedList {
        match list_type {
            ArcType::T1 => &mut self.t1,
            ArcType::T2 => &mut self.t2,
            ArcType::B1 => &mut self.b1,
            ArcType::B2 => &mut self.b2,
        }
    }

    #[inline]
    pub fn is_resident(list_type: ArcType) -> bool {
        matches!(list_type, ArcType::T1 | ArcType::T2)
    }

    #[inline]
    pub fn is_ghost(list_type: ArcType) -> bool {
        matches!(list_type, ArcType::B1 | ArcType::B2)
    }

    #[inline]
    fn increase_pivot_b1(&mut self) {
        let denominator = self.b1.len().max(1);
        let delta = (self.b1.len() / denominator).max(1);
        self.pivot = self.capacity.min(self.pivot.saturating_add(delta))
    }

    #[inline]
    fn increase_pivot_b2(&mut self) {
        let denominator = self.b2.len().max(1);
        let delta = (self.b2.len() / denominator).max(1);
        self.pivot = self.capacity.min(self.pivot.saturating_add(delta))
    }

    // ============================================================
    // EVICTION POLICY: DEBUGGING
    // ============================================================

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_validate_basic(&self) {
        debug_assert!(
            self.capacity > 0,
            "VARC capacity must be represented by a positive integer"
        );

        debug_assert!(
            self.pivot <= self.capacity,
            "VARC pivot must not exceed capacity: Pivot={}, Capacity={}",
            self.pivot,
            self.capacity
        );

        debug_assert!(
            self.resident_len() <= self.capacity,
            "VARC resident list must not exceed capacity: Pivot={}, Capacity={}",
            self.resident_len(),
            self.capacity
        );

        debug_assert!(
            self.ghost_len() <= self.capacity,
            "VARC ghost list must not exceed capacity: Pivot={}, Capacity={}",
            self.ghost_len(),
            self.capacity
        );

        debug_assert! {
            self.list_len() <= self.capacity.saturating_mul(2),
            "VARC complete history must not exceed 2x capacity: History={}, Capacity={}",
            self.list_len(),
            self.capacity
        };

        debug_assert_eq!(
            self.list_len(),
            self.index_map.len(),
            "VARC History/IndexMap length discrepancy"
        );
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_entry_match(&self, entry_id: &UniqueID, entry: ArcLocation) {
        let payload = self
            .list(entry.list)
            .get(entry.node)
            .expect("VARC ArcLocation must reference a live Node");

        debug_assert_eq!(
            payload,
            entry_id,
            "VARC ArcLocation & Payload mismatch"
        );
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_segment_match(&self, segment: ArcType, entry_id: &UniqueID) -> bool {
        let list = self.list(segment);
        let mut current = list.get_head();
        let mut visited = 0usize;
        let expected = list.len();

        while let Some(node) = current {
            let payload = list
                .get(node)
                .expect("VARC traversal encountered a non-live Node");

            if payload == entry_id {
                return true;
            }

            current = list.next_of(node);
            visited += 1;

            debug_assert!(
                visited <= expected,
                "VARC traversal exceeded expected length - Possible cycle"
            );
        }
        false
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertions_exclusivity(&self, entry_id: &UniqueID, entry: ArcLocation) {
        for arc_type in [ArcType::T1, ArcType::T2, ArcType::B1, ArcType::B2] {
            let contains = self.debug_segment_match(arc_type, entry_id);

            if arc_type == entry.list {
                debug_assert!(
                    contains,
                    "VARC entry missing from defined list"
                );
            } else {
                debug_assert!(
                    !contains,
                    "VARC entry appeared in multiple internal list"
                );
            }
        }
    }

    #[inline]
    #[cfg(debug_assertions)]
    fn debug_assertion_complete(&self) {
        self.debug_validate_basic();

        for (entry_id, entry) in &self.index_map {
            self.debug_assertions_entry_match(entry_id, *entry);
            self.debug_assertions_exclusivity(entry_id, *entry);
        }
    }
}