// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::SearchResult;

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D]; 
pub type SearchVector<const D: usize> = [u8; D];

pub type HeapResult = Vec<SearchResult>;

// ============================================================
// CUSTOM TYPE TRANSFORMATION
// ============================================================

pub fn f32_to_usize(value: f32) -> usize {
    value as usize
}

pub fn usize_to_f32(value: usize) -> f32 {
    value as f32
}

// ============================================================
// HELPER ENUMS
// ============================================================

pub enum VectorTier {
    Standard,
    Protected,
    Pinned,
}

pub enum ValidationMode {
    Lenient,
    Strict
}

pub enum DuplicatePolicy {
    Standard,
    Overwrite,
}