// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{result::SearchResult, utility::utils::UniqueID};

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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertOutcome {
    Inserted { id: UniqueID },
    InsertedWithEviction { id: UniqueID, evicted: UniqueID },
    DuplicateKept { existing: UniqueID },
    DuplicateReplaced { old: UniqueID, new: UniqueID },
    Rejected
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorTier {
    Standard,
    Protected,
    Pinned,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationMode {
    Lenient,
    Strict
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicatePolicy {
    KeepExisting,
    ReplaceExisting,
}