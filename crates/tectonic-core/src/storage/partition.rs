// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::shard::CacheShard;

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

#[allow(unused_variables)]
pub struct CachePartition {
    pub partition_id: usize,
    pub size: usize,
    pub capacity: usize,
    pub shards: Vec<CacheShard>,
}

#[allow(dead_code)]
impl CachePartition {
    pub fn with_capacity(partition_id: usize, capacity: usize, num_shards: usize) -> Self {
        if num_shards == 0 {
            return Self { 
                partition_id, 
                size: 0, 
                capacity: capacity, 
                shards: Vec::new() 
            };
        }
        
        let base = capacity / num_shards;
        let remainder = capacity % num_shards;

        let mut shards = Vec::with_capacity(num_shards);
        for shard_id in 0..num_shards {
            let shard_capacity = base + if shard_id < remainder { 1 } else { 0 };
            shards.push(CacheShard::with_capacity(shard_id, shard_capacity));
        }

        Self { 
            partition_id, 
            size: 0, 
            capacity, 
            shards 
        }
    }
}