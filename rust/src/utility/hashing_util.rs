use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn hash_u64(value: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_vector_id(vector: &[f32]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for &component in vector {
        // Convert f32 to u32 for hashing to maintain consistency.
        component.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}
