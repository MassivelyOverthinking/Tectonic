// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::collections::{HashMap, VecDeque};
use std::iter::repeat_with;
use std::usize;

use crate::error::TectonicError;
use crate::result::DimVector;
use crate::storage::location::{ArenaLocation, RepoLocation};
use crate::storage::partition::CachePartition;
use crate::storage::slot::RepoSlot;
use crate::utility::utils::calculate_sizes;

// ============================================================
// INTERNAL STORE (PARTITIONS + SHARDS)
// ============================================================

#[allow(dead_code)]
pub struct CacheRepo<const D: usize> {
    pub vector_repo: Vec<CachePartition<D>>,
    pub by_internal_id: Vec<RepoSlot>,
    pub by_user_id: HashMap<String, usize>,
    pub by_vector_hash: HashMap<u64, usize>,
    pub free_list: VecDeque<usize>,
    pub size: usize,
    pub capacity: usize,
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
            by_internal_id: repeat_with(|| RepoSlot::default()).take(max_entries).collect(),
            by_user_id: HashMap::new(),
            by_vector_hash: HashMap::new(),
            free_list: VecDeque::new(),
            capacity: max_entries,
            size: 0,
        }
    }

    pub fn insert(&mut self, vector: DimVector<D>) -> Result<bool, TectonicError> {
        !todo!()
    }
}
