// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::{BinaryHeap, VecDeque};

use crate::utility::typings::DimVector;
use crate::{error::TectonicError, search::{self, distance::SearchMethod}, storage::location::ArenaLocation};

// ============================================================
// INTERNAL SHARDS (MULTITHREADING)
// ============================================================

#[allow(dead_code)]
pub struct CacheShard {
    pub shard_id: u32,
    pub capacity: usize,
    pub size: usize,
    pub load_factor: f32,
    pub free_list: VecDeque<usize>,
    pub location_storage: Vec<Option<ArenaLocation<'static>>>
}

impl CacheShard {
    pub fn with_capacity(id: u32, capacity: usize) -> Self {
        Self { 
            shard_id: id,
            capacity: capacity,
            size: 0,
            load_factor: 0.0,
            free_list: VecDeque::new(),
            location_storage: vec![None; capacity as usize],
        }
    }

    pub fn insert(&mut self, location: ArenaLocation<'static>) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::RepoError { message: "Internal Shard is currently full!" });
        }

        if let Some(free_index) = self.free_list.pop_back() {
            self.location_storage[free_index] = Some(location);
            self.increment_and_update_factor();
            Ok(true)
        } else {
            self.location_storage[self.size] = Some(location);
            self.increment_and_update_factor();
            Ok(true)
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<ArenaLocation<'static>, TectonicError> {
        if index >= self.capacity {
            return Err(TectonicError::RepoError { message: 
                "Index out of bounds (Repository Shard)"
            });
        }

        if let Some(slot) = self.location_storage[index].take() {
            self.free_list.push_front(index);
            self.decrement_and_update_factor();
            Ok(slot)
        } else {
            return Err(TectonicError::RepoError { message: "Could not locate Location inside Repo" });
        }
    }

    pub fn get(&self, index: usize) -> Result<ArenaLocation<'static>, TectonicError> {
        if index >= self.capacity {
            return Err(TectonicError::RepoError { message: 
                "Index out of bounds (Repository Shard)"
            });
        }

        if let Some(slot) = &self.location_storage[index] {
            Ok(slot.clone())
        } else {
            return Err(TectonicError::RepoError { message: "Could not locate Location inside Repo" });
        }
    }

    fn increment_and_update_factor(&mut self) {
        self.size += 1;
        self.load_factor = self.size as f32 / self.capacity as f32
    }

    fn decrement_and_update_factor(&mut self) {
        self.size -= 1;
        self.load_factor = self.size as f32 / self.capacity as f32
    }

    fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}