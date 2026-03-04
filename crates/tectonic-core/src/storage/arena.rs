// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::VecDeque;
use std::iter::repeat_with;
use crate::storage::slot::ArenaSlot;
use crate::utility::utils::{VectorID};
use crate::result::{VectorEntry, DimVector};
use crate::error::TectonicError;

// ============================================================
// VECTOR STORAGE (ARENA)
// ============================================================

#[derive(Debug, Clone)]
pub struct VectorArena<const D: usize> {
    capacity: usize,
    size: usize,
    free_list: VecDeque<usize>,
    arena: Vec<ArenaSlot<D>>
}

#[allow(dead_code)]
impl<const D: usize> VectorArena<D> {
    fn insert(&mut self, value: DimVector<D>) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::CacheLimitError { size: self.size, limit: self.capacity })
        };

        if let Some(available_index) = self.free_list.pop_back() {
            let new_vector = VectorEntry::new(
                available_index,
                self.arena[available_index].get_and_increment(),
                value
            );
            self.arena[available_index].vector = Some(new_vector);
            self.size += 1;
            return Ok(true);
        } else {
            let next_index = self.size;
            let new_vector = VectorEntry::new(
                next_index,
                self.arena[next_index].get_and_increment(),
                value,
            );
            self.arena[next_index].vector = Some(new_vector);
            self.size += 1;
            return Ok(true);
        }
    }

    pub fn remove(&mut self, id: &usize, index: &usize) -> Result<bool, TectonicError> {
        let vector_id = *id;
        let vector_index = *index;

        if let Some(entry) = &self.arena[vector_index] {
            if entry.vector_id == vector_id {
                self.arena[vector_index] = None;
                self.size -= 1;
                self.free_list.push_front(vector_index);
                return Ok(true);
            } else {
                return Err(TectonicError::ArenaError { message: "IDs do not match!" });
            }
        }

        Err(TectonicError::ArenaError { message: "No Vector entry located at specified index" })
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

    pub fn with_capacity(max_entries: usize) -> Self {
        Self { 
            capacity: max_entries,
            size: 0,
            free_list: VecDeque::new(),
            arena: repeat_with(|| ArenaSlot::<D>::default()).take(max_entries).collect()
        }
    }
}