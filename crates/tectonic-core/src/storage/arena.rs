// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::BinaryHeap;
use std::iter::repeat_with;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::location::location_slab::LocationEntry;
use crate::storage::slot::ArenaSlot;
use crate::result::{CacheEntry, VectorEntry};
use crate::error::TectonicError;
use crate::utility::typings::{DimVector, TectonicResult};
use crate::utility::utils::UniqueID;
use crate::search::distance::SearchMethod;

// ============================================================
// VECTOR STORAGE (ARENA)
// ============================================================
// Fixed-size (static) vector Arena/Slab used as the primary storage for vector entries.
// ---
// `VectorArena` owns the actual vector data stored in the main cache. Each vector entry
// is placed into a stable slot-object, and granted a unique ID value for consistency-checking
// based on generational-counting.
// The VectorArena is designed to be a secure and stable storage layer for efficient vector management.
// ---
// The Arena/Slab structure utilizes a free-list to track availability  without removing existing
// resident vector entries. This provides the Arena/Slab with stable indicies for efficient
// vector retrieval, replacement, insertions and duplicate handling.
// --- 
// Invariants:
// 1. size <= capacity
// 2. free-list indicies always point to empty slots (vector == None)
// 3. free_list indcies are always within bounds of the arena vector (index < capacity)
// ---
// Performance:
// - Insert: O(1)
// - Remove: O(1)
// - Index Lookup: O(1)

#[derive(Debug, Clone)]
pub struct VectorArena<const D: usize> {
    capacity: usize,
    next_index: usize,
    size: usize,
    free_list: Vec<usize>,
    arena: Vec<ArenaSlot<D>>
}

// ============================================================
// VECTOR SLAB: CONSTRUCTORS
// ============================================================

#[allow(dead_code)]
impl<const D: usize> VectorArena<D> {
    pub fn with_capacity(max_entries: usize) -> TectonicResult<Self> {
        if max_entries == 0 {
            return Err(TectonicError::arena("Capacity must be greater than zero"));
        }

        Ok(Self { 
            capacity: max_entries,
            next_index: 0,
            size: 0,
            free_list: Vec::new(),
            arena: repeat_with(|| ArenaSlot::<D>::default()).take(max_entries).collect()
        })
    }

    // ============================================================
    // VECTOR SLAB: METHODS
    // ============================================================

    // Inserts new vector entry into the stable Arena/Slab structure and returns the index.
    // ---
    // The Arena/Slab first attempts to reuse previously freed slots by checking internal free-list.
    // If no reusable slots exists, the Arena/Slab proceeds to utilise the next available index.
    // ---
    // Returns:
    // - Ok(index) => The Arean/Slab index where the vector entry was inserted.
    // - Err(TectonicError) => If the Arena/Slab is full or due to inconsistencies.
    #[inline]
    pub fn insert(&mut self, value: DimVector<D>, metrics_enabled: bool) -> TectonicResult<(UniqueID, usize)> {
        // Check if the internal Slab/Arena structure is currently full.
        if self.is_full() {
            return Err(TectonicError::capacity_exceeded("Arena/Slab", self.size, self.capacity));
        }

        let index = if let Some(index) = self.free_list.pop() {
            index
        } else {
            self.get_next_index()
        };

        let slot = self.arena.get_mut(index)
        .ok_or(TectonicError::inconsistent_state("Arena insertion index out of bounds"))?;

        if slot.vector.is_some() {
            return Err(TectonicError::inconsistent_state("Arena attempted to insert into occupied slot"));
        }

        let generation = slot.get_and_increment();

        slot.vector = Some(VectorEntry::new(
            index,
            generation, 
            value, 
            metrics_enabled
        ));

        let inserted_id = UniqueID::new(index, generation);

        #[cfg(debug_assertions)]
        self.debug_assertions_validate()?;

        self.size += 1;
        Ok((inserted_id, index))
    }

    // Removes vector entry from the Arena/Slab based on provided index and unique ID.
    // ---
    // Removal validates the internal vector identity before freeing the slot. This validation 
    // check ensure that only the intended vector entry is removed, 
    // preventing accidental overwrites or data loss.
    // ---
    // Returns:
    // - Ok(true) => The vector entry was successfully removed.
    // - Err(TectonicError) => If the requested entry was not found or ID mismatch.
    pub fn remove(&mut self, id: UniqueID, index: usize) -> TectonicResult<bool> {
        let slot_value = self.arena.get_mut(index)
            .ok_or(TectonicError::arena("Index out of bounds"))?;

        let entry = slot_value.vector.as_ref()
            .ok_or(TectonicError::arena("Could not locate Vector inside Entry"))?;

        if *entry.get_unique_id() != id {
            return Err(TectonicError::inconsistent_state("Arena entry ID doesn't match found ID"));
        }

        slot_value.vector = None;     // Clear the slot by setting it to None.
        self.size -= 1;              // Decrease the size count of the Arena/Slab.
        self.free_list.push(index);   // Add the index of the removed entry to the Free

        #[cfg(debug_assertions)]
        self.debug_assertions_validate()?;

        Ok(true)
    }

    pub fn accurate_search<M>(&self, value: DimVector<D>, k: usize, search_method: &M) -> TectonicResult<Vec<CacheEntry<D>>>
    where M: SearchMethod<D> {
        if self.is_empty() {
            return Err(TectonicError::arena("Cache is currently empty"));
        };

        if k == 0 {
            return Ok(Vec::new());
        }

        let heap = self
            .arena
            .par_iter()
            .enumerate()
            .fold(
                || BinaryHeap::<CacheEntry<D>>::with_capacity(k + 1), 
                |mut local_heap, (arena_index, slot)| {
                    let Some(entry) = slot.vector.as_ref() else {
                        return local_heap;
                    };

                    let distance = search_method.distance_f32(&value, entry.get_vector());

                    if !distance.is_finite() {
                        #[cfg(debug_assertions)]
                        panic!("Non-finite distance calculated for vector at index {}: {}", arena_index, distance);

                        #[cfg(not(debug_assertions))]
                        return local_heap;
                    }

                    local_heap.push(CacheEntry::new(
                        arena_index,
                        entry.get_vector().clone(),
                        distance,
                    ));

                    if local_heap.len() > k {
                        local_heap.pop();
                    }

                    local_heap
                },
            )
            .reduce(
                || BinaryHeap::<CacheEntry<D>>::with_capacity(k + 1), 
                |mut global_heap, local_heap| {
                    for candidate in local_heap {
                        global_heap.push(candidate);

                        if global_heap.len() > k {
                            global_heap.pop();
                        }
                    }

                    global_heap
                },
            );

        let results: Vec<CacheEntry<D>> = heap.into_sorted_vec();

        #[cfg(debug_assertions)]
        {
            debug_assert!(results.len() <= k, "Expcted result length doesn't match k");
            debug_assert!(results.len() <= self.size, "Expected result length exceeds number of entries in Arena");
        }

        Ok(results)
    }

    pub fn get_vector_by_location(&self, location: &LocationEntry, id: UniqueID) -> TectonicResult<(&DimVector<D>, &UniqueID)> {
        let arena_entry = self.arena.get(*location.get_arena())
            .ok_or(TectonicError::arena("Index out of bounds"))?;

        let entry = arena_entry.vector.as_ref()
            .ok_or(TectonicError::arena("Could not locate Vector inside Entry"))?;

        if *entry.get_unique_id() == id {
            Ok((entry.get_vector(), entry.get_unique_id()))
        } else {
            return Err(TectonicError::inconsistent_state("Arena entry ID doesn't match found ID"));
        }
    }

    pub fn get_vector_at_position(&self, index: usize) -> TectonicResult<(&DimVector<D>, &UniqueID)> {
        // Helper-method
        // Retrieves reference-pointer to the interanl VectorEntry located in parameter: Index.
        // Used for Duplicate-handling & Vector retrieval.

        // Retrieve Mutable instance of the internal Slot (ArenaSlot).
        // Default => Throw new TectonicError::ArenaError
        let arena_entry = self.arena.get(index)
            .ok_or(TectonicError::arena("Index out of bounds"))?;

        // Retrieve Mutable instance of the actual VectorEntry-instance found inside Slot.
        // Default => Throw new TectonicError::ArenaError
        let entry = arena_entry.vector.as_ref()
            .ok_or(TectonicError::arena("Could not locate Vector inside Entry"))?;

        Ok((entry.get_vector(), entry.get_unique_id()))   // Return Borrowed-instance of the internal VectorEntry.
    }

    pub fn replace_vector(&mut self, new_vector: DimVector<D>, location: &LocationEntry) ->TectonicResult<UniqueID> {
        // Helper-method
        // Replaces the internal VectorEntry-instance with new value found by ArenaLocation.
        // Used for Duplicate-handling.

        // Retrieve Mutable instance of the internal Slot (ArenaSlot).
        // Default => Throw new TectonicError::ArenaError
        let arena_entry = self.arena.get_mut(*location.get_arena())
            .ok_or(TectonicError::arena("Index out of bounds"))?;

        // Retrieve Mutable instance of the actual VectorEntry-instance found inside Slot.
        // Default => Throw new TectonicError::ArenaError
        let entry = arena_entry.vector.as_mut()
            .ok_or(TectonicError::arena("Could not locate Vector inside Entry"))?;

        // Use internal Helper-method to replace the internal VectorEntry.
        let vector_id = entry.replace_internal_vector(new_vector);
        Ok(vector_id)       // Returns Vector-ID for clarity & debugging.
    }

    #[inline]
    pub fn update_vector(&mut self, new_vector: DimVector<D>, index: &usize) -> TectonicResult<UniqueID> {
        // Helper-method
        // Replaces the internal VectorEntry-instance with new value found by ArenaLocation.
        // Used for Duplicate-handling.

        // Retrieve Mutable instance of the internal Slot (ArenaSlot).
        // Default => Throw new TectonicError::ArenaError
        let arena_entry = self.arena.get_mut(*index)
            .ok_or(TectonicError::arena("Index out of bounds"))?;

        // Retrieve Mutable instance of the actual VectorEntry-instance found inside Slot.
        // Default => Throw new TectonicError::ArenaError
        let entry = arena_entry.vector.as_mut()
            .ok_or(TectonicError::arena("Could not locate Vector inside Entry"))?;

        // Use internal Helper-method to replace the internal VectorEntry.
        let vector_id = entry.replace_internal_vector(new_vector);
        Ok(vector_id)       // Returns Vector-ID for clarity & debugging.
    }

    #[inline]
    pub fn clear(&mut self) -> TectonicResult<bool> {
        self.arena.iter_mut().for_each(|slot| slot.vector = None);
        self.free_list.clear();
        self.next_index = 0;
        self.size = 0;

        #[cfg(debug_assertions)]
        self.debug_assertions_validate()?;

        Ok(true)
    }

    // ============================================================
    // VECTOR SLAB: HELPER METHODS
    // ============================================================

    pub fn load_factor(&self) -> f32 {
        // Helper-method for checking the current availability of the Arena/Slab.
        (self.size as f32 / self.capacity as f32) * 100.0
    }

    pub fn size(&self) -> usize {
        // Helper-method for determining the current number of entries in the Arena/Slab.
        self.size
    }

    pub fn capacity(&self) -> usize {
        // Helper-method for determining the maximum number of entries the Arena/Slab can hold.
        self.capacity
    }

    pub fn remain_capacity(&self) -> usize {
        // Helper-method for determining the remaining capacity of the Arena/Slab.
        self.capacity - self.size
    }

    pub fn is_full(&self) -> bool {
        // Helper-method for determining if the Arena/Slab is currently full.
        self.size >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        // Helper-method for determining if the Arena/Slab is currently empty.
        self.size == 0
    }

    pub fn increase_size(&mut self) {
        self.size += 1;
    }

    pub fn get_next_index(&mut self) -> usize {
        let current_index = self.next_index;
        self.next_index += 1;
        current_index
    }

    // ============================================================
    // VECTOR SLAB: DEBUGGING
    // ============================================================

    pub fn debug_assertions_validate(&self) -> TectonicResult<()> {
        if self.size > self.capacity {
            return Err(TectonicError::inconsistent_state("Size exceeds capacity in Arena"));
        }

        let occupied = self
            .arena
            .iter()
            .filter(|slot| slot.vector.is_some())
            .count();

        if occupied != self.size {
            return Err(TectonicError::inconsistent_state("Arena size does not match occupied slot count"));
        }

        for &index in &self.free_list {
            if index >= self.capacity {
                return Err(TectonicError::inconsistent_state("Free-list contains out-of-bounds index"));
            }

            if self.arena[index].vector.is_some() {
                return Err(TectonicError::inconsistent_state("Free-list contains occupied slot"));
            }
        }

        Ok(())
    }
}