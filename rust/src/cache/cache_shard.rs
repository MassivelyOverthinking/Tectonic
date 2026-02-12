use crate::vector::vector_entry::VectorEntry;

#[derive(Clone)]
#[allow(dead_code)]
pub struct CacheShard<const D: usize> {
    /// Unique identifier for the cache shard (Immutable).
    pub shard_id: u64,

    /// Maximum number of entries this shard can hold (Immutable).
    pub max_entries: usize,

    /// Current number of entries stored in the shard (Mutable).
    pub entry_count: usize,

    /// Internal storage for cache partitions (Mutable).
    pub entries: Vec<VectorEntry<D>>,
}

#[allow(dead_code)]
impl <const D: usize> CacheShard<D> {
    pub fn new(shard_id: u64, max_entries: usize) -> Self {
        Self {
            shard_id,
            max_entries,
            entry_count: 0,
            entries: Vec::with_capacity(max_entries),
        }
    }

    pub fn get_shard_centroid(&self) -> Option<([f32; D], f32)> {
        let count = self.entry_count as f32;
        if count == 0.0 {
            return None;
        }

        let mut mean = [0.0f32; D];

        for entry in &self.entries {
            for index in 0..D {
                mean[index] += entry.vector[index];
            }
        };

        Some((mean, count))
    }

    fn is_full(&self) -> bool {
        self.entry_count >= self.max_entries
    }
    
}