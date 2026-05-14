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
    #[inline]
    pub fn new(id: UniqueID, entry: QuantizedEntry) -> Self {
        Self { 
            unique_id: id,
            search_vector: entry,
        }
    }

    #[inline]
    pub fn get_id(&self) -> &UniqueID {
        &self.unique_id
    }

    #[inline]
    pub fn get_vector(&self) -> &QuantizedEntry {
        &self.search_vector
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Repolocation {
    partition_index: usize,
    shard_index: usize,
    slot_index: usize,
}

impl Repolocation {
    #[inline]
    pub fn new(partition: usize, shard: usize, slot: usize) -> Self {
        Self { 
            partition_index: partition, 
            shard_index: shard, 
            slot_index: slot 
        }
    }

    #[inline]
    pub fn get_partition_index(&self) -> &usize {
        &self.partition_index
    }

    #[inline]
    pub fn get_shard_index(&self) -> &usize {
        &self.shard_index
    }

    #[inline]
    pub fn get_slot_index(&self) -> &usize {
        &self.slot_index
    }
}