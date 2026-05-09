// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{quantization::quantized_entry::QuantizedEntry, utility::{typings::{DimVector, Hash64}, utils::UniqueID}};

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

#[derive(Debug, Clone)]
pub struct BootstrapEntry<const D: usize> {
    vector: DimVector<D>,
    quantized: QuantizedEntry,
    internal_id: UniqueID,
    vector_hash: Hash64,
}

impl<const D: usize> BootstrapEntry<D> {
    #[inline]
    pub fn new(
        vector: DimVector<D>,
        quantized: QuantizedEntry,
        internal_id: UniqueID,
        vector_hash: Hash64
    ) -> Self {
        Self {
            vector,
            quantized,
            internal_id,
            vector_hash
        }
    }

    #[inline]
    pub fn get_vector(&self) -> &DimVector<D> {
        &self.vector
    }

    #[inline]
    pub fn get_quantized_entry(&self) -> &QuantizedEntry {
        &self.quantized
    }

    #[inline]
    pub fn get_unique_id(&self) -> &UniqueID {
        &self.internal_id
    }

    #[inline]
    pub fn get_vector_hash(&self) -> &Hash64 {
        &self.vector_hash
    }

    #[inline]
    pub fn get_vector_mut(&mut self) -> &mut DimVector<D> {
        &mut self.vector
    }

    #[inline]
    pub fn get_quantized_entry_mut(&mut self) -> &mut QuantizedEntry {
        &mut self.quantized
    }

    #[inline]
    pub fn get_internal_id_mut(&mut self) -> &mut UniqueID {
        &mut self.internal_id
    }

    #[inline]
    pub fn get_vector_hash_mut(&mut self) -> &mut Hash64 {
        &mut self.vector_hash
    }
}