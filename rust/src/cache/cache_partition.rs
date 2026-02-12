use crate::vector::vector_entry::VectorEntry;
use crate::cache::cache_shard::CacheShard;

#[derive(Clone)]
#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    /// Unique identifier for the cache partition (Immutable).
    pub partition_id: u64,

    /// Maximum number of vectors this partition can hold (Immutable).
    pub max_entries: usize,

    /// Current number of vectors stored in the partition (Mutable).
    pub entry_count: usize,

    /// K-means centroids representing the partition's vector clusters (Mutable).
    pub centroid: Option<Vec<[f32; D]>>,

    /// Internal storage for vector entries (Mutable).
    pub entries: Vec<VectorEntry<D>>,

    /// Internal storage for cache shards (Mutable).
    pub shards: Vec<CacheShard<D>>,
}

#[allow(dead_code)]
impl<const D: usize> CachePartition<D> {
    pub fn new(partition_id: u64, max_entries: usize, shard_count: usize) -> Self {
        Self {
            partition_id,
            max_entries,
            entry_count: 0,
            centroid: None,
            entries: Vec::with_capacity(max_entries),
            shards: Vec::with_capacity(shard_count),
        }
    }

    pub fn query(&self, _vector: &[f32; D], _top_k: usize) -> Vec<(u64, f32)> {
        // Placeholder for actual query logic.
        Vec::new()
    }

    pub fn insert(&mut self, _entry: VectorEntry<D>) -> Result<(), String> {
        // Placeholder for actual insert logic.
        Ok(())
    }

    pub fn metrics(&self) -> String {
        // Placeholder for metrics implementation.
        "Partition metrics not implemented".to_string()
    }

    pub fn update_centroid(&mut self){
        assert!(self.shards.is_empty(), "Cannot update centroid for an empty partition");

        let mut total_entries = 0;
        let mut mean = [0.0f32; D];
        
        for shard in &self.shards {
            if let Some(centroid) = shard.get_shard_centroid() {
                total_entries += centroid.1 as usize;
                for (index, value) in centroid.0.iter().enumerate() {
                    mean[index] += *value;
                }
            }
            
        }

        mean.iter_mut().for_each(|x| *x /= total_entries as f32);
        self.centroid = Some(vec![mean]);
    }

    fn is_full(&self) -> bool {
        self.entry_count >= self.max_entries
    }
}