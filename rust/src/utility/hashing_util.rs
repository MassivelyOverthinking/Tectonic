use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn hash_u64(value: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
