use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn hash_u64(value: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_vector_id<const D: usize>(vector: &[u8; D]) -> u64 {
    let mut hasher = DefaultHasher::new();
    vector.hash(&mut hasher);
    hasher.finish()
}

pub fn generate_vector_id<const D: usize>(vector: &[u8; D]) -> u64 {
        hash_vector_id(vector)
}
