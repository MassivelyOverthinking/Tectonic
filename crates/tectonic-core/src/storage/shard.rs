// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{collections::{BinaryHeap, VecDeque}};

use crate::{error::TectonicError, quantization::quantized_entry::QuantizedEntry, result::SearchResult, search::distance::{SearchMethod}, storage::location::ArenaLocation, utility::typings::{HeapResult, usize_to_f32}};

// ============================================================
// INTERNAL SHARDS (MULTITHREADING)
// ============================================================

#[allow(dead_code)]
pub struct CacheShard<const D: usize> {
    pub shard_id: u32,
    pub capacity: usize,
    pub size: usize,
    pub load_factor: f32,
    pub free_list: VecDeque<usize>,
    pub location_storage: Vec<Option<ArenaLocation>>
}

#[allow(dead_code)]
impl<const D: usize> CacheShard<D> {
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

    pub fn insert(&mut self, location: ArenaLocation) -> Result<usize, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::RepoError { message: "Internal Shard is currently full!" });
        }

        if let Some(free_index) = self.free_list.pop_back() {
            self.location_storage[free_index] = Some(location);
            self.increment_and_update_factor();
            Ok(free_index)
        } else {
            let free_index = self.size;
            self.location_storage[free_index] = Some(location);
            self.increment_and_update_factor();
            Ok(free_index)
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<ArenaLocation, TectonicError> {
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

    pub fn search<M>(
        &self, 
        vector: &QuantizedEntry, 
        search_method: &M, 
        k: usize
    ) -> Result<HeapResult, TectonicError>
    where M: SearchMethod<D> {
        if self.is_empty() || k == 0 {
            return Ok(Vec::new());
        }

        let mut binary_heap: BinaryHeap<SearchResult> = BinaryHeap::with_capacity(k);

        for loc_opt in self.location_storage.iter() {
            let location = match loc_opt {
                Some(location) => location,
                None => continue,
            };

            let location_vector = location.get_vector();
            let location_index = location.get_index();
            let distance_value = search_method.distance_u8(vector, location_vector);

            let result = SearchResult::new(location_index, &distance_value);

            if binary_heap.len() < k {
                binary_heap.push(result);
            } else if let Some(worst_case) = binary_heap.peek() {
                if result.distance < worst_case.distance {
                    binary_heap.pop();
                    binary_heap.push(result);
                }
            }
        }

        let ordered_array = binary_heap.into_sorted_vec();
        Ok(ordered_array)
    }

    fn increment_and_update_factor(&mut self) {
        self.size += 1;
        self.load_factor = usize_to_f32(self.size) / usize_to_f32(self.capacity)
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