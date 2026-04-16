// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry, utility::utils::UniqueID};

// ============================================================
// CACHE LOCATION
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ArenaLocation {
    entry_id: usize,
    entry_index: usize,
    search_vector: QuantizedEntry,
}

#[allow(dead_code)]
impl ArenaLocation {
    pub fn new(id: usize, index: usize, vector: QuantizedEntry) -> Self {
        Self { 
            entry_id: id, 
            entry_index: index,
            search_vector: vector,
        }
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ShardEntry {
    unique_id: UniqueID,
    search_vector: QuantizedEntry,
}

impl ShardEntry {
    pub fn new(id: UniqueID, entry: QuantizedEntry) -> Self {
        Self { 
            unique_id: id,
            search_vector: entry,
        }
    }

    pub fn get_id(&self) -> &UniqueID {
        &self.unique_id
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