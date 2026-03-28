// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::VecDeque;
use std::iter::repeat_with;
use crate::storage::location::ArenaLocation;
use crate::storage::slot::ArenaSlot;
use crate::result::{VectorEntry};
use crate::error::TectonicError;
use crate::utility::typings::DimVector;
use crate::utility::utils::UniqueID;

// ============================================================
// VECTOR STORAGE (ARENA)
// ============================================================

#[derive(Debug, Clone)]
pub struct VectorArena<const D: usize> {
    capacity: usize,
    pub size: usize,
    free_list: VecDeque<usize>,
    pub arena: Vec<ArenaSlot<D>>
}

#[allow(dead_code)]
impl<const D: usize> VectorArena<D> {
    pub fn insert(&mut self, value: DimVector<D>, metrics_enabled: bool) -> Result<usize, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::CacheLimitError { size: self.size, limit: self.capacity })
        };

        if let Some(available_index) = self.free_list.pop_back() {
            let new_vector = VectorEntry::new(
                available_index,
                self.arena[available_index].get_and_increment(),
                value,
                metrics_enabled,
            );
            self.arena[available_index].vector = Some(new_vector);
            self.size += 1;
            return Ok(available_index);
        } else {
            let next_index = self.size;
            let new_vector = VectorEntry::new(
                next_index,
                self.arena[next_index].get_and_increment(),
                value,
                metrics_enabled,
            );
            self.arena[next_index].vector = Some(new_vector);
            self.size += 1;
            return Ok(next_index);
        }
    }

    pub fn remove(&mut self, id: &usize, index: &usize) -> Result<bool, TectonicError> {
        let vector_id = *id;
        let vector_index = *index;

        if let Some(entry) = &self.arena[vector_index].vector {
            if entry.vector_id.slot_id == vector_id {
                self.arena[vector_index].vector = None;
                self.size -= 1;
                self.free_list.push_front(vector_index);
                return Ok(true);
            } else {
                return Err(TectonicError::ArenaError { message: "IDs do not match!" });
            }
        }

        Err(TectonicError::ArenaError { message: "No Vector entry located at specified index" })
    }

    pub fn get_vector_by_location(&self, location: &ArenaLocation) -> Result<(&DimVector<D>, &UniqueID), TectonicError> {
        let arena_entry = self.arena.get(*location.get_index())
            .ok_or(TectonicError::ArenaError {
                message: "Index out of bounds" 
            })?;

        let entry = arena_entry.vector.as_ref()
            .ok_or(TectonicError::ArenaError { 
                message: "Could not locate Vector inside Entry" 
            })?;

        if entry.vector_id.slot_id == *location.get_entry_id() {
            Ok((&entry.vector, &entry.vector_id))
        } else {
            return Err(TectonicError::InconsistenStateError {
                message: "Arena entry ID doesn't match found ID" 
            });
        }
    }

    pub fn get_vector_at_position(&self, index: usize) -> Result<&DimVector<D>, TectonicError> {
        let arena_entry = self.arena.get(index)
            .ok_or(TectonicError::ArenaError {
                message: "Index out of bounds" 
            })?;

        let entry = arena_entry.vector.as_ref()
            .ok_or(TectonicError::ArenaError { 
                message: "Could not locate Vector inside Entry" 
            })?;

        Ok(&entry.vector)
    }

    pub fn replace_vector(&mut self, new_vector: DimVector<D>, location: &ArenaLocation) ->Result<UniqueID, TectonicError> {
        let arena_entry = self.arena.get_mut(*location.get_index())
            .ok_or(TectonicError::ArenaError {
                message: "Index out of bounds" 
            })?;

        let entry = arena_entry.vector.as_mut()
            .ok_or(TectonicError::ArenaError { 
                message: "Could not locate Vector inside Entry" 
            })?;

        let vector_id = entry.replace_internal_vector(new_vector);
        Ok(vector_id)
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