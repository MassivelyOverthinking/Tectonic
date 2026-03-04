// ============================================================
// IMPORTS AND MODULES
// ============================================================

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