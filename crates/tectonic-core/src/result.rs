// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone)]
pub struct VectorEntry<const D: usize> {
    vector_id: usize,
    vector: [f32; D]
}

impl<const D: usize> VectorEntry<D> {
    pub fn new(id: usize, vector: [f32; D]) -> Self {
        Self { 
            vector_id: id,
            vector
        }
    }
}