// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::utils::DimVector;

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VectorEntry<const D: usize> {
    pub vector_id: usize,
    pub vector: DimVector<D>,
}

impl<const D: usize> VectorEntry<D> {
    pub fn new(id: usize, vector: DimVector<D>) -> Self {
        Self { 
            vector_id: id,
            vector
        }
    }
}