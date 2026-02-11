use crate::vector::vector_entry::VectorEntry;

#[derive(Clone)]
#[allow(dead_code)]
pub struct CachePartition<const D: usize> {
    /// Unique identifier for the cache partition (Immutable).
    partition_id: u64,

    /// Maximum number of vectors this partition can hold (Immutable).
    max_entries: usize,

    /// Current number of vectors stored in the partition (Mutable).
    entry_count: usize,

    /// K-means centroids representing the partition's vector clusters (Mutable).
    centroid: Option<Vec<[f32; D]>>,

    /// Internal storage for vector entries (Mutable).
    entries: Vec<VectorEntry<D>>,
}

impl<const D: usize> CachePartition<D> {
    pub fn new(partition_id: u64, max_entries: usize) -> Self {
        Self {
            partition_id,
            max_entries,
            entry_count: 0,
            centroid: None,
            entries: Vec::new(),
        }
    }

    pub fn update_centroid(&mut self){
        let count = self.entry_count as f32;
        if count == 0.0 {
            self.centroid = None;
            return;
        }

        let mut mean = [0.0f32; D];

        for entry in &self.entries {
            for index in 0..D {
                mean[index] += entry.vector[index];
            }
        };

        let new_centroid = mean.map(|x| x / count);

        self.centroid = Some(vec![new_centroid]);
        
    }

    fn is_full(&self) -> bool {
        self.entry_count >= self.max_entries
    }
}