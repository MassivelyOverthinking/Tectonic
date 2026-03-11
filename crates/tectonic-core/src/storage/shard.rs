// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{error::TectonicError, storage::location::ArenaLocation};

// ============================================================
// INTERNAL SHARDS (MULTITHREADING)
// ============================================================

#[allow(dead_code)]
pub struct CacheShard {
    pub shard_id: u32,
    pub capacity: usize,
    pub size: usize,
    pub load_factor: f32,
    pub location_storage: Vec<Option<ArenaLocation<'static>>>
}

impl CacheShard {
    pub fn with_capacity(id: u32, capacity: usize) -> Self {
        Self { 
            shard_id: id,
            capacity: capacity,
            size: 0,
            load_factor: 0.0,
            location_storage: vec![None; capacity as usize],
        }
    }

    pub fn insert(&mut self, location: ArenaLocation) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::RepoError { message: "Internal Shard is currently full!" });
        }

        self.location_storage[self.size] = Some(location);
        self.size += 1;
        self.update_load_factor();
        Ok(true)
    }

    fn update_load_factor(&mut self) {
        self.load_factor = self.size as f32 / self.capacity as f32
    }

    fn is_full(&self) -> bool {
        self.size >= self.capacity
    }
}