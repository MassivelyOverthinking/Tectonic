// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// QUANTIZATION ENTRY
// ============================================================

pub struct QuantizedEntry {
    pub vector: Vec<u8>,
}

impl QuantizedEntry {
    pub fn new(vector: Vec<u8>) -> Self {
        Self { vector }
    }
}