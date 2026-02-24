// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VectorEntry<const D: usize> {
    pub vector_id: usize,
    pub vector: [f32; D]
}

impl<const D: usize> VectorEntry<D> {
    pub fn new(id: usize, vector: [f32; D]) -> Self {
        Self { 
            vector_id: id,
            vector
        }
    }
}