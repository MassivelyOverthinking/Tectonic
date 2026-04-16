// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry, utility::{typings::DimVector, utils::UniqueID}};

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

#[derive(Debug, Clone)]
pub struct BootstrapEntry<const D: usize> {
    pub vector: DimVector<D>,
    pub quantized: QuantizedEntry,
    pub internal_id: UniqueID,
    pub vector_hash: u64,
}