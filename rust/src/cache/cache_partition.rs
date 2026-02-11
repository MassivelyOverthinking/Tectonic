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
    centroid: Vec<[f32; D]>,

    /// Internal storage for vector entries (Mutable).
    entries: Vec<VectorEntry<D>>,
}