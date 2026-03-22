// ============================================================
// IMPORTS AND MODULES
// ============================================================

use core::f32;
use std::collections::{HashMap, VecDeque};
use std::iter::repeat_with;
use std::usize;

use crate::error::TectonicError;
use crate::quantization::quantized_entry::QuantizedEntry;
use crate::result::VectorResult;
use crate::utility::router::BootstrapEntry;
use crate::utility::typings::DimVector;
use crate::search::distance::{SearchMethod, SearchMethodDyn};
use crate::storage::location::{ArenaLocation, RepoLocation};
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

    // Centroid Buffer State
    pub centroid_buffer: Vec<BootstrapEntry<D>>,
    pub centroid_buffer_threshold: usize,
    pub centroids_initialized: bool,
}

#[allow(dead_code)]
impl<const D: usize> CacheRepo<D> {
    pub fn with_capacity(max_entries: usize, partitions: usize, shards: usize) -> Self {
        let partition_capacities = calculate_sizes(max_entries, partitions);

        let mut partitions_vector = Vec::with_capacity(partition_capacities.len());
        for (id, &cap) in partition_capacities.iter().enumerate() {
            partitions_vector.push(CachePartition::with_capacity( id as u32, cap as u64, shards as u32));
        }

        let buffer_threshold = (partitions * 16).max(partitions);

        Self {
            vector_repo: partitions_vector,
            by_internal_id: repeat_with(|| RepoSlot::default()).take(max_entries).collect(),
            by_user_id: HashMap::new(),
            by_vector_hash: HashMap::new(),
            free_list: VecDeque::new(),
            capacity: max_entries,
            size: 0,

            // Buffer State & Initialization
            centroid_buffer: Vec::with_capacity(buffer_threshold),
            centroid_buffer_threshold: buffer_threshold,
            centroids_initialized: false,
        }
    }

    pub fn insert(&mut self, vector: &DimVector<D>, user_id: Option<&str>, overwrite: bool) -> Result<bool, TectonicError> {
        if self.is_full() {
            return Err(TectonicError::RepoError { message: "Vector Repository is currently full!" });
        }

        let vector_hash = hash_dimvector(vector);
        todo!()
    }

    pub fn get_by_vector_id(&self, id: usize) -> Result<ArenaLocation<'static>, TectonicError> {
        if id < 0 || id >= self.by_internal_id.len() {
            return Err(TectonicError::RepoError { message: "Provided ID is out of bounds!" });
        }
        todo!()
    }

    pub fn get_by_user_id(&self, id: &str) -> Result<ArenaLocation<'static>, TectonicError> {
        todo!()
    }

    fn search(
        &self, 
        quanttized_vector: &QuantizedEntry,
        standard_vecor: &DimVector<D>,
        search_method: &dyn SearchMethodDyn<D>, 
        k: usize,
        search_partitions: usize,
    ) -> Result<VectorResult<D>, TectonicError> {
        if self.is_empty() {
            return Err(TectonicError::RepoError { message: "Vector repository is currently empty" });
        }
        todo!()

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

    pub fn find_nearest_centroids<M>(&self, vector: &DimVector<D>, top_n: usize, distance: &M) -> Result<Vec<usize>, TectonicError> 
    where M: SearchMethod<D> {
        
        if self.vector_repo.is_empty() {
            return Err(TectonicError::RepoError { message: "No internal partitions found!" });
        }

        if top_n == 0 {
            return Ok(Vec::new());
        }

        let mut candidates: Vec<(usize, f32)> = Vec::with_capacity(self.vector_repo.len());

        for (position, partition) in self.vector_repo.iter().enumerate() {
            if let Some(par_centroid) = partition.centroid.as_ref() {
                let centroid_distance = distance.distance_f32(vector, &par_centroid);
                candidates.push((position, centroid_distance));
            } else {
                continue;
            }
        }

        
        if candidates.is_empty() {
            return Err(TectonicError::RepoError { message: "No partitions with centroids found!" });
        }

        candidates.sort_unstable_by(|x, y| {
            x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let limit = top_n.min(candidates.len());

        let result = candidates
            .into_iter()
            .take(limit)
            .map(|(position, _)| position)
            .collect();

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

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
