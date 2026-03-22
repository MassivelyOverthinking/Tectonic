// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry, utility::typings::DimVector};

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

#[derive(Debug, Clone)]
pub struct BootstrapEntry<const D: usize> {
    pub vector: DimVector<D>,
    pub quantized: QuantizedEntry,
    pub internal_id: usize,
    pub user_id: Option<String>,
    pub vector_hash: u64,
}