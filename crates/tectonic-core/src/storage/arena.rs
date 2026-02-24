// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::VecDeque;
use crate::utility::utils::VectorID;
use crate::{error::TectonicError, result::VectorEntry};

// ============================================================
// VECTOR STORAGE (ARENA)
// ============================================================

#[derive(Debug, Clone)]
pub struct VectorArena<const D: usize> {
    next_index: usize,
    capacity: usize,
    size: usize,
    id: VectorID,
    free_list: VecDeque<usize>,
    arena: [VectorEntry<D>; D]
}

impl<const D: usize> VectorArena<D> {

    fn insert(&mut self, value: [f32; D]) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::CacheLimitError { size: self.size, limit: self.capacity })
        };

        if let Some(available_index) = self.free_list.pop_back() {
            let new_vector = VectorEntry::new(
                self.id.get_and_increment(),
                value
            );
            self.arena[available_index] = new_vector;
            self.size += 1;
            return Ok(true);
        } else {
            let next_index = self.next_index;
            let new_vector = VectorEntry::new(
                self.id.get_and_increment(),
                value
            );
            self.arena[next_index] = new_vector;
            self.next_index += 1;
            self.size += 1;
            return Ok(true);
        }
    }

    pub fn load_factor(&self) -> f32 {
        (self.size as f32 / self.capacity as f32) * 100.0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.size > self.capacity
    }
}