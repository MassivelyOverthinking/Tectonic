// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::error::TectonicError;
use crate::result::DimVector;
use crate::storage::shard::{CacheShard};
use crate::storage::location::{ArenaLocation};
use crate::utility::utils::calculate_sizes;

// ============================================================
// INTERNAL PARTITIONS (SEARCH SPACE)
// ============================================================

#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    pub partition_id: u32,
    pub centroid: Option<[f32; D]>,
    pub size: u64,
    pub capacity: u64,
    pub shards: Vec<CacheShard>,
}

#[allow(dead_code)]
impl<const D: usize> CachePartition<D> {
    pub fn with_capacity(partition_id: u32, capacity: u64, num_shards: u32) -> Self {
        let shard_sizes = calculate_sizes(capacity as usize, num_shards as usize);

        let mut shard_vectors = Vec::with_capacity(shard_sizes.len());
        for (id, &cap) in shard_sizes.iter().enumerate() {
            shard_vectors.push(CacheShard::with_capacity(id as u32, cap as u64));
        }

        Self { 
            partition_id,
            centroid: None,
            size: 0, 
            capacity, 
            shards: shard_vectors,
        }
    }

    pub fn insert(&self, _location: ArenaLocation) -> Result<bool, TectonicError> {
        !todo!()
    }

    #[inline]
    fn add_centroid_vector(&mut self, vector: &DimVector<D>) -> Result<bool, TectonicError> {
        if self.centroid.is_some() {
            return Err(TectonicError::CentroidError { message: "Centroid is already initialized!" });
        }

        self.centroid = Some(*vector);
        self.size += 1;

        Ok(true)
    }

    #[inline]
    fn moving_centroid_average(&mut self, vector: &DimVector<D>) -> Result<bool, TectonicError> {
        let centroid = self.centroid.as_mut().ok_or_else(|| {
            TectonicError::CentroidError { message: "No centroid available!" }
        })?;

        let old_n = self.size;
        let new_n = old_n + 1;
        let inv_new_avg = 1.0f32 / (new_n as f32);

        for index in 0..D {
            centroid[index] = (centroid[index] * old_n as f32 + vector[index]) * inv_new_avg;
        }

        self.size = new_n;
        Ok(true)
    }
}