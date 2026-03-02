// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::shard::CacheShard;
use crate::utility::utils::calculate_sizes;

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    pub partition_id: usize,
    pub centroid: Option<[f32; D]>,
    pub size: usize,
    pub capacity: usize,
    pub shards: Vec<CacheShard>,
}

#[allow(dead_code)]
impl<const D: usize> CachePartition<D> {
    pub fn with_capacity(partition_id: usize, capacity: usize, num_shards: usize) -> Self {
        let shard_sizes = calculate_sizes(capacity, num_shards);

        let mut shard_vectors = Vec::with_capacity(shard_sizes.len());
        for (id, &cap) in shard_sizes.iter().enumerate() {
            shard_vectors.push(CacheShard::with_capacity(id, cap));
        }

        Self { 
            partition_id,
            centroid: None,
            size: 0, 
            capacity, 
            shards: shard_vectors,
        }
    }
}