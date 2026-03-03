// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::usize;

use crate::error::TectonicError;
use crate::result::DimVector;
use crate::storage::partition::CachePartition;
use crate::utility::utils::calculate_sizes;

// ============================================================
// INTERNAL STORE (PARTITIONS + SHARDS)
// ============================================================

#[allow(dead_code)]
pub struct CacheRepo<const D: usize> {
    pub vector_repo: Vec<CachePartition<D>>,
}

#[allow(dead_code)]
impl<const D: usize> CacheRepo<D> {
    pub fn with_capacity(max_entries: usize, partitions: usize, shards: usize) -> Self {
        let partition_capacities = calculate_sizes(max_entries, partitions);

        let mut partitions_vector = Vec::with_capacity(partition_capacities.len());
        for (id, &cap) in partition_capacities.iter().enumerate() {
            partitions_vector.push(CachePartition::with_capacity( id, cap, shards));
        }

        Self {
            vector_repo: partitions_vector,
        }
    }

    pub fn insert(&mut self, vector: DimVector<D>) -> Result<bool, TectonicError> {
        !todo!()
    }
}
