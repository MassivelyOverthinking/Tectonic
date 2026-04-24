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
}