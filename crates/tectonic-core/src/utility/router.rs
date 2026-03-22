// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::typings::DimVector;

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

#[derive(Debug, Clone)]
pub struct BootstrapEntry<const D: usize> {
    pub vector: DimVector<D>,
    pub internal_id: usize,
    pub user_id: Option<String>,
    pub vector_hash: u64,
}