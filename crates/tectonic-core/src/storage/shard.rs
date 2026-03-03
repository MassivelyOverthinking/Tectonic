// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::location::Location;

// ============================================================
// INTERNAL SHARDS (MULTITHREADING)
// ============================================================

#[allow(dead_code)]
pub struct CacheShard {
    pub shard_id: usize,
    pub capacity: usize,
    pub size: usize,
    pub location_storage: Vec<Option<Location<'static>>>
}

impl CacheShard {
    pub fn with_capacity(id: usize, capacity: usize) -> Self {
        Self { 
            shard_id: id,
            capacity: capacity,
            size: 0,
            location_storage: vec![None; capacity],
        }
    }
}