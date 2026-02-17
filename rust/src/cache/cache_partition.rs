use crate::vector::vector_entry::VectorEntry;
use crate::cache::cache_shard::CacheShard;
use crate::utility::hashing_util::generate_vector_id;
use crate::utility::vector_utils::scalar_quantize;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    /// Unique identifier for the cache partition (Immutable).
    pub partition_id: u64,

    /// Atomic counter for generating unique vector entry IDs (Mutable).
    pub id_counter: Arc<AtomicUsize>,

    /// Maximum number of vectors this partition can hold (Immutable).
    pub max_entries: usize,

    /// Current number of vectors stored in the partition (Mutable).
    pub entry_count: usize,

    /// K-means centroids representing the partition's vector clusters (Mutable).
    pub centroid: Option<[f32; D]>,

    /// ID map for quick lookup of vector entries (Mutable).
    pub id_map: HashMap<u64, [u8; D]>,

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
            id_counter: Arc::new(AtomicUsize::new(0)),
            max_entries,
            entry_count: 0,
            centroid: None,
            id_map: HashMap::new(),
            entries: Vec::with_capacity(max_entries),
            shards: Vec::with_capacity(shard_count),
        }
    }

    pub fn query(&self, _vector: &[f32; D], _top_k: usize) -> Vec<(u64, f32)> {
        // Placeholder for actual query logic.
        Vec::new()
    }

    pub fn insert(&mut self, entry: &[f32], overwrite: bool) -> Result<bool, Err> {
        // Placeholder for actual insert logic.
        assert!(self.is_full(), "Cannot insert into a full partition");

        let quantized_vector = scalar_quantize(entry, 256);
        let map_id = generate_vector_id(&quantized_vector);

        if self.id_map.contains_key(&map_id) {
            if !overwrite {
                let existing_vector = self.id_map.get(&map_id).unwrap();
                if existing_vector == &quantized_vector {
                    return Err(false); // Duplicate entry, insertion failed.
                }
            self.id_map.remove(&map_id);
            self.entry_count -= 1;
            }
        };

        self.id_map.insert(map_id, quantized_vector);
        self.entries.push(entry);
        self.entry_count += 1;
        Ok(true)
    }

    pub fn metrics(&self) -> String {
        // Placeholder for metrics implementation.
        "Partition metrics not implemented".to_string()
    }

    fn calculate_shard_size(max_entries: usize, shard_count: usize) -> Vec<usize> {
        // Base Case -> No shards defined.
        assert!(shard_count > 0, "Shard count must be greater than 0");

        // Evenly distribute max_entries across shards.
        let base = max_entries / shard_count;
        let remainder = max_entries % shard_count;

        // Allocate reamainders to individual shards to ensure total matches max_entries.
        let mut sizes = vec![base; shard_count as usize];
        for i in 0..remainder as usize {
            sizes[i] += 1;
        }

        // Return calculated shard sizes.
        sizes
    }

    pub fn initiate_shards(&mut self, total_size: usize, shard_count: usize) {
        // Calcuate shard sizes based on total partition size and number of shards.
        let sizes = Self::calculate_shard_size(total_size, shard_count);
        
        // Initialize shards with calculated sizes and unique shard IDs.
        for (shard_id, size) in sizes.iter().enumerate() {
            self.shards.push(CacheShard::new(shard_id as u64, *size));
        }
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
        self.centroid = Some(mean);
    }

    fn is_full(&self) -> bool {
        self.entry_count >= self.max_entries
    }
}