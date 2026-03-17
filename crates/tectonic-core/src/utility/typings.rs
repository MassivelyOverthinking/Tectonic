// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::SearchResult;

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D]; 
pub type SearchVector<const D: usize> = [i8; D];

pub type HeapResult = Option<Vec<SearchResult>>;

// ============================================================
// CUSTOM TYPE TRANSFORMATION
// ============================================================

pub fn f32_to_usize(value: u32) -> usize {
    value as usize
}

pub fn usize_to_f32(value: usize) -> f32 {
    value as f32
}