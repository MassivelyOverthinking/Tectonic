// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{collections::{BinaryHeap, VecDeque}};

use crate::{error::TectonicError, location::location_entry::ShardEntry, quantization::quantized_entry::QuantizedEntry, result::SearchResult, search::distance::SearchMethod, utility::{typings::{HeapResult, TectonicResult, usize_to_f32}, utils::UniqueID}};

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
    pub location_storage: Vec<Option<ShardEntry>>
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

    pub fn insert(&mut self, entry: ShardEntry) -> TectonicResult<usize> {
        if self.is_full() {
            return Err(TectonicError::repository("Internal Shard is currently full!"));
        }

        if let Some(free_index) = self.free_list.pop_back() {
            self.location_storage[free_index] = Some(entry);
            self.increment_and_update_factor();
            Ok(free_index)
        } else {
            let free_index = self.size;
            self.location_storage[free_index] = Some(entry);
            self.increment_and_update_factor();
            Ok(free_index)
        }
    }

    pub fn remove(&mut self, index: usize) -> TectonicResult<UniqueID> {
        if index >= self.capacity {
            return Err(TectonicError::repository("Index out of bounds (Repository Shard)"));
        }

        if let Some(entry) = self.location_storage[index].take() {
            self.free_list.push_front(index);
            self.decrement_and_update_factor();
            Ok(entry.get_id().clone())
        } else {
            return Err(TectonicError::repository("Could not locate Location inside Repo"));
        }
    }

    pub fn search<M>(
        &self, 
        vector: &QuantizedEntry, 
        search_method: &M, 
        k: usize
    ) -> TectonicResult<HeapResult>
    where M: SearchMethod<D> {
        if self.is_empty() || k == 0 {
            return Ok(Vec::new());
        }

        let mut binary_heap: BinaryHeap<SearchResult> = BinaryHeap::with_capacity(k);

        for shard_entry in self.location_storage.iter() {
            let entry = match shard_entry {
                Some(entry) => entry,
                None => continue,
            };

            let search_vector = entry.get_vector();
            let entry_id = entry.get_id();
            let distance_value = search_method.distance_u8(vector, search_vector);

            let result = SearchResult::new(entry_id, &distance_value);

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

    #[inline]
    pub fn clear(&mut self) -> TectonicResult<bool> {
        self.location_storage.iter_mut().for_each(|entry| *entry = None);
        self.free_list.clear();
        self.size = 0;
        self.load_factor = 0.0;
        
        Ok(true)
    }
}