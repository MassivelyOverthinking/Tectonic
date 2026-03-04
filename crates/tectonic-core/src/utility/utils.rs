// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// GENERAL UTILITY METHODS & STRUCTS
// ============================================================

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::result::DimVector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct UniqueID {
    pub slot_id: usize,
    pub gen_id: u32,
}

#[allow(dead_code)]
impl UniqueID {
    pub fn new(slot: usize, generation: u32) -> Self {
        Self {
            slot_id: slot,
            gen_id: generation,
        }
    }
}

pub fn calculate_sizes(max_entries: usize, elements: usize) -> Vec<usize> {
    let base_value = max_entries / elements;
    let remainder_value = max_entries % elements;

    let mut sizes = vec![base_value; elements];

    for size in &mut sizes[..remainder_value] {
        *size += 1;
    }

    sizes
}

#[allow(dead_code)]
pub fn hash_dimvector<const D: usize>(vector: &DimVector<D>) -> u64 {
    let mut vec_hash = DefaultHasher::new();
    for &value in vector.iter() {
        value.to_bits().hash(&mut vec_hash);
    }

    vec_hash.finish()
}