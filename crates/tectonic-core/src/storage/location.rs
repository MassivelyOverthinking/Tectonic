// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CACHE LOCATION
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ArenaLocation {
    user_id: Option<String>,
    entry_id: usize,
    entry_index: usize,
    search_vector: QuantizedEntry,
}

#[allow(dead_code)]
impl ArenaLocation {
    pub fn new(user_id: Option<String>, id: usize, index: usize, vector: QuantizedEntry) -> Self {
        Self { 
            user_id: user_id,
            entry_id: id, 
            entry_index: index,
            search_vector: vector,
        }
    }

    pub fn get_user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    pub fn get_entry_id(&self) -> &usize {
        &self.entry_id
    }

    pub fn get_index(&self) -> &usize {
        &self.entry_index
    }

    pub fn get_vector(&self) -> &QuantizedEntry {
        &self.search_vector
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct RepoLocation {
    pub partition_idx: usize,
    pub shard_idx: usize,
    pub slot_idx: usize,
}

#[allow(dead_code)]
impl RepoLocation {
    pub fn new(partition: usize, shard: usize, slot: usize) -> Self {
        Self { 
            partition_idx: partition, 
            shard_idx: shard, 
            slot_idx: slot 
        }
    }

    pub fn get_partition_index(&self) -> &usize {
        &self.partition_idx
    }

    pub fn get_shard_index(&self) -> &usize {
        &self.shard_idx
    }

    pub fn get_slot_index(&self) -> &usize {
        &self.slot_idx
    }
}