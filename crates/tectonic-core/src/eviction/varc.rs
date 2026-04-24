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

#[allow(dead_code)]
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
    fn decrease_pivot_b2(&mut self) {
        let denominator = self.b2.len().max(1);
        let delta = (self.b2.len() / denominator).max(1);
        self.pivot = self.capacity.min(self.pivot.saturating_add(delta))
    }

    #[inline]
    fn get_replacement_target(list_type: ArcType) -> ArcType {
        match list_type {
            ArcType::T1 => ArcType::B1,
            ArcType::T2 => ArcType::B2,
            ArcType::B1 | ArcType::B2 => unreachable!("Ghost list cannot get replacement")
        }
    }

    #[inline]
    fn remove_from_location(&mut self, entry_id: &UniqueID, location: ArcLocation) -> Option<UniqueID> {
        let removed_value = self.list_mut(location.list).unlink(location.node)?;
        let removed_from_map = self.index_map.remove(entry_id);

        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                &removed_value,
                entry_id,
                "VARC payload mismatch - Wrong internal value removed"
            );
            debug_assert!(
                removed_from_map.is_some(),
                "VARC removed from internal list but not from IndexMap"
            );
            self.debug_assertion_complete();
        }

        Some(removed_value)
    }

    #[inline]
    fn choose_replacement_source(&self) -> Option<ArcType> {
        if !self.t1.is_empty() && (self.t1.len() > self.pivot || self.t2.is_empty()) {
            Some(ArcType::T1)
        } else if !self.t2.is_empty() {
            Some(ArcType::T2)
        } else if !self.t1.is_empty() {
            Some(ArcType::T1)
        } else {
            None
        }
    }

    #[inline]
    fn remove_ghost_lru(&mut self, list_type: ArcType) -> Option<UniqueID> {
        debug_assert!(Self::is_ghost(list_type));

        let removed_value= self.list_mut(list_type).pop_front()?;
        let removed_from_map = self.index_map.remove(&removed_value);

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                removed_from_map.is_some(),
                "VARC Ghost entry existed in internal list, but not in IndexMap"
            );
        }
        Some(removed_value)
    }

    #[inline]
    fn trim_ghost_lists(&mut self) {
        while self.ghost_len() > self.capacity {
            if self.b1.len() >= self.b2.len() && !self.b1.is_empty() {
                let _ = self.remove_ghost_lru(ArcType::B1);
            } else if !self.b2.is_empty() {
                let _ = self.remove_ghost_lru(ArcType::B2);
            } else {
                break;
            }
        }

        while self.list_len() > self.capacity.saturating_mul(2) {
            if !self.b1.is_empty() {
                let _ = self.remove_ghost_lru(ArcType::B1);
            } else if !self.b2.is_empty() {
                let _ = self.remove_ghost_lru(ArcType::B2);
            } else {
                break;
            }
        }
    }

    #[inline]
    fn move_entry_to_ghost(&mut self, from: ArcType, to: ArcType) -> Option<UniqueID> {
        debug_assert!(Self::is_resident(from));
        debug_assert!(Self::is_ghost(to));

        let victim = self.list_mut(from).pop_front()?;
        let ghost_node = self.list_mut(to).push_front(victim);

        let old_value = self.index_map.insert(
            victim, 
            ArcLocation { 
                list: to, 
                node: ghost_node, 
            }
        );

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                old_value.is_some(),
                "VARC victim entry already existed in IndexMap"
            );
            debug_assert!(
                self.list(to).is_tail(ghost_node),
                "VARC Ghost node insertion did not reach the correct position"
            );
        }

        self.trim_ghost_lists();

        #[cfg(debug_assertions)]
        self.debug_assertion_complete();

        Some(victim)
    }

    #[inline]
    fn move_entry(&mut self, from: ArcType, to: ArcType, entry_id: &UniqueID, entry: NodeValue) -> Option<()> {
        let removed_value = self.list_mut(from).unlink(entry)?;

        debug_assert_eq!(
            removed_value,
            *entry_id,
            "VARC requested entry did not match removed payload"
        );

        let new_node = self.list_mut(to).push_back(removed_value);

        let old_value = self.index_map.insert(
            removed_value, 
            ArcLocation { 
                list: to, 
                node: new_node, 
            },
        );

        #[cfg(debug_assertions)]
        {
            debug_assert!(
                old_value.is_some(),
                "VARC expected an existing entry"
            );
            debug_assert!(
                self.list(to).is_tail(new_node),
                "VARC Ghost node insertion did not reach the correct position"
            );
            self.debug_assertion_complete();
        }
        Some(())
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

impl EvictionStrategy for VARC {
    #[inline]
    fn on_get(&mut self, entry_id: &UniqueID) {
        let location = match self.index_map.get(entry_id).copied() {
            Some(location) => location,
            None => {
                #[cfg(debug_assertions)]
                self.debug_assertion_complete();
                return;
            }
        };

        #[cfg(debug_assertions)]
        {
            self.debug_assertion_complete();
            self.debug_assertions_entry_match(entry_id, location);
        }

        match location.list {
            ArcType::T1 => {
                let _ = self.move_entry(ArcType::T1, ArcType::T2, entry_id, location.node);
            },
            ArcType::T2 => {
                let moved_value = self.t2.move_to_back(location.node);

                #[cfg(debug_assertions)]
                {
                    debug_assert!(
                        moved_value.is_some(),
                        "VARC T2 hit failed to correctly move entry to new position"
                    );
                    debug_assert!(
                        self.t2.is_tail(location.node),
                        "VARC T2 hit moved value to incorrect position"
                    );
                    self.debug_assertion_complete();
                }
            },
            ArcType::B1 | ArcType::B2 => {
                #[cfg(debug_assertions)]
                self.debug_assertion_complete();
            }
        }
    }

    #[inline]
    fn on_remove(&mut self, entry_id: &UniqueID) -> Option<UniqueID> {
        let location = self.index_map.get(entry_id).copied()?;

        #[cfg(debug_assertions)]
        {
            self.debug_assertion_complete();
            self.debug_assertions_entry_match(entry_id, location);
        }

        self.remove_from_location(entry_id, location)
    }

    #[inline]
    fn on_insert(&mut self, entry: UniqueID) {
        let location = self.index_map.get(&entry).copied();

        match location {
            Some(ArcLocation { list: ArcType::T1, node }) => {
                let _ = self.move_entry(ArcType::T1, ArcType::T2, &entry, node);
            }
            Some(ArcLocation { list: ArcType::T2, node }) => {
                let _ = self.t2.move_to_back(node);

                #[cfg(debug_assertions)]
                self.debug_assertion_complete();
            }
            Some(ArcLocation { list: ArcType::B1, node }) => {
                self.increase_pivot_b1();

                debug_assert!(
                    self.resident_len() < self.capacity,
                    "VARC B1 admission requires available resident capacity; call evict_victim() first"
                );

                let _ = self.move_entry(ArcType::B1, ArcType::T2, &entry, node);
            }
            Some(ArcLocation { list: ArcType::B2, node }) => {
                self.decrease_pivot_b2();

                debug_assert!(
                    self.resident_len() < self.capacity,
                    "VARC B2 admission requires available resident capacity; call evict_victim() first"
                );

                let _ = self.move_entry(ArcType::B2, ArcType::T2, &entry, node);
            }
            None => {
                debug_assert!(
                    self.resident_len() < self.capacity,
                    "VARC cold admission requires available resident capacity; call evict_victim() first"
                );

                let node = self.t1.push_back(entry);
                let old = self.index_map.insert(
                    entry,
                    ArcLocation {
                        list: ArcType::T1,
                        node,
                    },
                );

                #[cfg(debug_assertions)]
                {
                    debug_assert!(
                        old.is_none(),
                        "VARC cold insert unexpectedly replaced existing entry"
                    );
                    debug_assert!(
                        self.t1.is_tail(node),
                        "VARC cold insert did not land at T1 MRU"
                    );
                    self.debug_assertion_complete();
                }
            }
        }

        self.trim_ghost_lists();

        #[cfg(debug_assertions)]
        self.debug_assertion_complete();
    }

    #[inline]
    fn get_victim(&mut self) -> Option<&UniqueID> {
        #[cfg(debug_assertions)]
        self.debug_assertion_complete();

        match self.choose_replacement_source()? {
            ArcType::T1 => self.t1.front(),
            ArcType::T2 => self.t2.front(),
            ArcType::B1 | ArcType::B2 => unreachable!("Ghost lists can be choosen as replacement source")
        }
    }

    #[inline]
    fn evict_victim(&mut self) -> Option<UniqueID> {
        #[cfg(debug_assertions)]
        self.debug_assertion_complete();

        let source = self.choose_replacement_source()?;
        let target = Self::get_replacement_target(source);
        let victim = self.move_entry_to_ghost(source, target)?;

        #[cfg(debug_assertions)]
        self.debug_assertion_complete();

        Some(victim)
    }

    #[inline]
    fn len(&self) -> usize {
        self.resident_len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.resident_len() == 0
    }
}