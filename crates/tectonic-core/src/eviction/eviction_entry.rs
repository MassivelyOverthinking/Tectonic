// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::utils::UniqueID;

// ============================================================
// EVICTION ENTRY
// ============================================================

#[derive(Debug, Clone)]
struct EvictionLocation {
    shard_idx: usize,
    slot_idx: usize,
}

impl EvictionLocation {
    pub fn new(shard_idx: usize, slot_idx: usize) -> Self {
        Self {
            shard_idx,
            slot_idx 
        }
    }

    pub fn get_shard_index(&self) -> &usize {
        &self.shard_idx
    }

    pub fn get_slot_index(&self) -> &usize {
        &self.slot_idx
    }
}

#[derive(Debug, Clone)]
struct EvictionData {
    average_distance: f64,
    access_count: usize,
    hit_count: usize,
    miss_count: usize,
    last_accessed: u64,
    score: f64,
}

impl Default for EvictionData {
    fn default() -> Self {
        Self {
            average_distance: 0.0,
            access_count: 0,
            hit_count: 0,
            miss_count: 0,
            last_accessed: 0,
            score: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvictionEntry {
    entry_id: UniqueID,
    partition_id: usize,
    location: EvictionLocation,
    created_at: u64,
    data: EvictionData,
}
