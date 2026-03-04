// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::location::ArenaLocation;

// ============================================================
// INTERNAL SHARDS (MULTITHREADING)
// ============================================================

#[allow(dead_code)]
pub struct CacheShard {
    pub shard_id: u32,
    pub capacity: u64,
    pub size: u64,
    pub location_storage: Vec<Option<ArenaLocation<'static>>>
}

impl CacheShard {
    pub fn with_capacity(id: u32, capacity: u64) -> Self {
        Self { 
            shard_id: id,
            capacity: capacity,
            size: 0,
            location_storage: vec![None; capacity as usize],
        }
    }
}