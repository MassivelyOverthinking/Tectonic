// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{collections::HashMap};
use crate::{error::TectonicError, utility::{typings::{Hash64, TectonicResult}, utils::UniqueID}};

// ============================================================
// GLOBAL LOCATION ARENA/SLAB
// ============================================================
// Global index-slab mapping vector IDs and vector hashes to physical cache locations.
// ---
// `LocationSlab` is the source of truth for resolving:
// - `UniqueID      => LocationEntry`
// - `vector_hash   => UniqueID         => LocationEntry`
// ---
// Location-slab supports pending entries so vectors can be inserted into the arena during
// repository bootstrap before partition/shard/slot locations exist.

#[derive(Debug)]
pub struct LocationSlab {
    storage: HashMap<UniqueID, LocationEntry>,
    hashes: HashMap<u64, UniqueID>
}

impl Default for LocationSlab {
    #[inline]
    fn default() -> Self {
        Self { 
            storage: HashMap::new(),
            hashes: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
impl LocationSlab {
    #[inline]
    pub fn insert_routed(
        &mut self, 
        id: UniqueID, 
        hash: Hash64,
        arena_index: usize,
        partition_index: usize,
        shard_index: usize,
        slot_index: usize,
    ) -> TectonicResult<()> {
        if self.storage.contains_key(&id) {
            return Err(TectonicError::location(
                "Location ID already exists in location slab"
            ));
        };

        if self.hashes.contains_key(&hash) {
            return Err(TectonicError::location(
                "Vector hash already exists in location slab"
            ));
        };
    }

    #[inline]
    pub fn add_location(&mut self, id: &UniqueID, hash: u64, entry: LocationEntry) {
        self.storage.insert(*id, entry);
        self.hashes.insert(hash, *id);
    }

    #[inline]
    pub fn get_location(&self, id: &UniqueID) -> Option<&LocationEntry> {
        self.storage.get(id)
    }

    #[inline]
    pub fn contains_location(&self, id: &UniqueID) -> bool {
        self.storage.contains_key(id)
    }

    #[inline]
    pub fn get_by_hash(&self, hash: u64) -> Option<&LocationEntry> {
        if let Some(found_id) = self.hashes.get(&hash) {
            self.storage.get(found_id)
        } else {
            None
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.storage.clear();
        self.hashes.clear();
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LocationEntry {
    id: UniqueID,
    hash: Hash64,
    arena_index: usize,
    state: LocationState,
    partition_index: usize,
    shard_index: usize,
    slot_index: usize,
    arena_index: usize,
}

#[allow(dead_code)]
impl LocationEntry {
    #[inline]
    fn new(partition: usize, shard: usize, slot: usize, arena: usize) -> Self {
        Self {
            partition_index: partition,
            shard_index: shard,
            slot_index: slot,
            arena_index: arena
        }
    }

    #[inline]
    pub fn get_partition(&self) -> &usize {
        &self.partition_index
    }

    #[inline]
    pub fn get_shard(&self) -> &usize {
        &self.shard_index
    }

    #[inline]
    pub fn get_slot(&self) -> &usize {
        &self.slot_index
    }

    #[inline]
    pub fn get_arena(&self) -> &usize {
        &self.arena_index
    }

    #[inline]
    pub fn set_partition(&mut self, index: usize) {
        self.partition_index = index;
    }

    #[inline]
    pub fn set_shard(&mut self, index: usize) {
        self.shard_index = index;
    }

    #[inline]
    pub fn set_slot(&mut self, index: usize) {
        self.slot_index = index;
    }

    #[inline]
    pub fn set_arena(&mut self, index: usize) {
        self.arena_index = index;
    }
}

// Routing state for cached vectors.
// ---
// During bootstrapping, vectors exist in the location arena before repository routing is
// finalized. Those entries are represented as `Pending`.
// --- 
// Once centroids are initialized and the repository routes the entry into a
// partition/shard/slot, the location becomes `Routed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationState {
        Routed,
        Pending {
            partition_index: usize,
            shard_index: usize,
            slot_index: usize,
        }
    }