// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::utils::UniqueID;

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D]; 

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VectorEntry<const D: usize> {
    pub vector_id: UniqueID,
    pub vector: DimVector<D>,
}

impl<const D: usize> VectorEntry<D> {
    pub fn new(id: usize, generation: u32, vector: DimVector<D>) -> Self {
        Self { 
            vector_id: UniqueID::new(id, generation),
            vector
        }
    }
}