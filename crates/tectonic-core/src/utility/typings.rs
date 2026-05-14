// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::location::location_entry::Repolocation;
use crate::{result::SearchResult, utility::utils::UniqueID};
use crate::error::TectonicError;

// ============================================================
// CUSTOM DATA-TYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D];
pub type Hash64 = u64; 
pub type NodeValue = usize;

pub type HeapResult = Vec<SearchResult>;
pub type TectonicResult<T> = Result<T, TectonicError>;

// ============================================================
// CUSTOM TYPE TRANSFORMATION
// ============================================================

#[allow(dead_code)]
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
    DuplicateReplaced { id: UniqueID },
    Rejected
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RepoInsertOutcome {
    Buffered,
    Routed {
        location: Repolocation
    },
    Bootstrapped {
        routed: Vec<(UniqueID, Repolocation)>,
    },
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Accurate,
    Approximate,
    ApproximateRerank,
}