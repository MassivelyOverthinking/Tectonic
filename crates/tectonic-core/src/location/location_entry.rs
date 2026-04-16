// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry, utility::utils::UniqueID};

// ============================================================
// CACHE LOCATION
// ============================================================

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