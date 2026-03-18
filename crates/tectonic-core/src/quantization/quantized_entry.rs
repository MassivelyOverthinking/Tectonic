// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::slice::Iter;

// ============================================================
// QUANTIZATION ENTRY
// ============================================================

#[derive(Debug, Clone)]
pub struct QuantizedEntry {
    pub vector: Vec<u8>,
}

impl QuantizedEntry {
    pub fn new(vector: Vec<u8>) -> Self {
        Self { vector }
    }

    pub fn get_iter(&self) -> Iter<'_, u8> {
        self.vector.iter()
    }

    pub fn get_length(&self) -> usize {
        self.vector.len()
    }
}