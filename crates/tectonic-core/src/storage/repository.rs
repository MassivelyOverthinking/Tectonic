// ============================================================
// IMPORTS AND MODULES
// ============================================================

use core::f32;
use std::collections::{HashMap, VecDeque};
use std::iter::repeat_with;
use std::usize;

use crate::error::TectonicError;
use crate::result::DimVector;
use crate::search::distance::{SearchMethod};
use crate::storage::location::{RepoLocation};
use crate::storage::partition::CachePartition;
use crate::storage::slot::RepoSlot;
use crate::utility::utils::{calculate_sizes, hash_dimvector};

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
            partitions_vector.push(CachePartition::with_capacity( id as u32, cap as u64, shards as u32));
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

    pub fn insert(&mut self, vector: &DimVector<D>, user_id: Option<&str>, overwrite: bool) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::RepoError { message: "Vector Repository is currently full!" });
        }

        let vec_hash = hash_dimvector(vector);
        if let Some(found_hash) = self.by_vector_hash.get(&vec_hash) {
            if !overwrite {
                let found_loc = self.find_arena_by_hash(found_hash)?;
                let found_vec = self.get_id_by_location(found_loc)?;
                return Ok(false);
            } else {
                return Ok(true);
            }
        } else {
            Ok(true)
        }
    }

    pub fn get_id_by_location(&self, location: &RepoLocation) -> Result<usize, TectonicError> {
        let arena_loc = self.vector_repo[*location.get_partition_index()]
            .shards[*location.get_shard_index()]
            .location_storage[*location.get_slot_index()]
            .as_ref()
            .ok_or_else(|| TectonicError::RepoError { message: "No Location located!" })?;

        let arena_index = arena_loc.get_index();
        Ok(*arena_index)
    }

    pub fn find_nearest_centroid(&self, vector: &DimVector<D>, distance: &dyn SearchMethod<D>) -> Result<usize, TectonicError> {
        if self.vector_repo.is_empty() {
            return Err(TectonicError::RepoError { message: "No internal partitions found!" });
        }

        let mut result = 0;
        let mut shortest_distance = f32::MAX;

        for (position, partition) in self.vector_repo.iter().enumerate() {
            if let Some(par_centroid) = partition.centroid.as_ref() {
                let centroid_distance = distance.distance(vector, &par_centroid);
                if centroid_distance <= shortest_distance {
                    shortest_distance = centroid_distance;
                    result = position
                }
            } else {
                continue;
            }
        }

        Ok(result)
    }

    #[inline]
    pub fn find_arena_by_hash(&self, value: &usize) -> Result<&RepoLocation, TectonicError> {
        if let Some(found_location) = self.by_internal_id[*value].location.as_ref() {
            Ok(found_location)
        } else {
            Err(TectonicError::RepoError { message: "Could not find RepoLocation!" })
        }
    }

    #[inline]
    pub fn is_vectors_equal(&self, x: &DimVector<D>, y: &DimVector<D>) -> bool {
        x == y
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }
}
