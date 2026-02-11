use crate::utility::hashing_util;

#[derive(Clone)]
#[repr(align(32))]
pub struct VectorEntry<const D: usize> {
    /// Unique identifier for the vector entry (Immutable).
    pub entry_id: u64,

    /// High-dimensional vector data (Immutable).
    pub vector: [f32; D],

    /// Unique hash-value for entry key (Immutable).
    pub key_hash: u64,
}

impl <const D: usize> VectorEntry<D> {
    pub fn new(id: u64, vector: [f32; D]) -> Self {
        let hash_key = hashing_util::hash_u64(id);
        Self {
            entry_id: id,
            vector,
            key_hash: hash_key,
        }
    }
}