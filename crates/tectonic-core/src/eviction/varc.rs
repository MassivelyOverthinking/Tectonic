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