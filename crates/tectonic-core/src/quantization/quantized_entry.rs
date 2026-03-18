// ============================================================
// IMPORTS AND MODULES
// ============================================================

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
}