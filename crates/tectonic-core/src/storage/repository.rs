// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::usize;

use crate::storage::partition::CachePartition;

// ============================================================
// INTERNAL STORE (PARTITIONS + SHARDS)
// ============================================================

pub struct CacheRepo {
    pub vector_repo: Vec<CachePartition>,
}

impl CacheRepo {
    pub fn with_capacity(_max_entries: usize, _partitions: usize, _shards: usize) -> Self {
        !todo!()
    }

    fn calculate_partition_sizes(max_entries: usize, partitions: usize) -> Vec<usize> {
        let base_value = max_entries / partitions;
        let remainder_value = max_entries % partitions;

        let mut sizes = vec![base_value; partitions];

        for size in &mut sizes[..remainder_value] {
            *size += 1;
        }

        sizes
    }

}
