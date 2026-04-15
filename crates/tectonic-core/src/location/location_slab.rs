// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::HashMap;
use crate::utility::utils::UniqueID;

// ============================================================
// GLOBAL LOCATION ARENA/SLAB
// ============================================================

#[derive(Debug)]
pub struct LocationSlab {
    storage: HashMap<UniqueID, LocationEntry>
}

#[derive(Debug)]
pub struct LocationEntry {
    partition_index: usize,
    shard_index: usize,
    slot_index: usize,
    arena_index: usize
}

impl LocationEntry {
    fn new(partition: usize, shard: usize, slot: usize, arena: usize) -> Self {
        Self {
            partition_index: partition,
            shard_index: shard,
            slot_index: slot,
            arena_index: arena
        }
    }

    pub fn get_partition(&self) -> &usize {
        &self.partition_index
    }

    pub fn get_shard(&self) -> &usize {
        &self.shard_index
    }

    pub fn get_slot(&self) -> &usize {
        &self.slot_index
    }

    pub fn get_arena(&self) -> &usize {
        &self.arena_index
    }

    pub fn set_partition(&mut self, index: usize) {
        self.partition_index = index;
    }

    pub fn set_shard(&mut self, index: usize) {
        self.shard_index = index;
    }

    pub fn set_slot(&mut self, index: usize) {
        self.slot_index = index;
    }

    pub fn set_arena(&mut self, index: usize) {
        self.arena_index = index;
    }
}