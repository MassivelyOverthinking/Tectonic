// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::error::TectonicError;
use crate::storage::location::ArenaLocation;
use crate::utility::typings::DimVector;

// ============================================================
// GENERAL UTILITY METHODS & STRUCTS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct UniqueID {
    pub slot_id: usize,
    pub gen_id: u32,
}

impl Display for UniqueID  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.gen_id, self.slot_id)
    }
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

#[allow(dead_code)]
pub fn hash_arena_location(location: &ArenaLocation) -> u64 {
    let mut loc_hash = DefaultHasher::new();

    location.get_entry_id().hash(&mut loc_hash);
    location.get_index().hash(&mut loc_hash);

    loc_hash.finish()
}

#[allow(dead_code)]
pub fn secondary_arena_hash(hash: u64) -> u64 {
    hash.rotate_left(32) ^ 0x9e3779b97f4a7c15
}

pub fn validate_vector<const D: usize>(vector: &DimVector<D>) -> Result<(), TectonicError> {
    for index in 0..D {
        let value = vector[index];
        if !value.is_finite() {
            return Err(TectonicError::InvalidVectorError { 
                index: index 
            })
        }
    }
    Ok(())
}