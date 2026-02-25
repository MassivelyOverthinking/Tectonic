// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::shard::CacheShard;

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

pub struct CachePartition {
    pub partition_id: usize,
    pub size: usize,
    pub capacity: usize,
    pub shards: Vec<CacheShard>,
}

impl CachePartition {
    pub fn with_capacity(_capacity: usize, _num_shards: usize) -> Self {
        !todo!()
    }

    fn calculate_shard_sizes(capacity: usize, shards: usize) -> Vec<usize> {
        let base_value = capacity / shards;
        let remainder_value = capacity % shards;

        let mut sizes = vec![base_value; shards];

        for size in &mut sizes[..remainder_value] {
            *size += 1;
        }

        sizes
    }
}