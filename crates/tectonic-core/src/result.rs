// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{metrics::vector_metric::VectorMetrics, utility::utils::UniqueID};
use crate::utility::typings::DimVector;
use std::cmp::Ordering;
use std::usize;

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VectorEntry<'a, const D: usize> {
    pub vector_id: UniqueID,
    pub user_id: Option<&'a str>,
    pub vector: DimVector<D>,
    pub metrics: Option<VectorMetrics<D>>
}

impl<'a, const D: usize> VectorEntry<'a, D> {
    pub fn new(id: usize, generation: u32, user_id: Option<&'a str>, vector: DimVector<D>, metrics_enabled: bool) -> Self {
        Self { 
            vector_id: UniqueID::new(id, generation),
            user_id: user_id,
            vector,
            metrics: if metrics_enabled { Some(VectorMetrics::default()) } else { None },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VectorResult<const D: usize> {
   pub size: usize,
   pub vectors: Vec<DimVector<D>>,
}

impl<const D: usize> VectorResult<D> {
    pub fn new(num_vectors: usize, result_vectors: Vec<DimVector<D>>) -> Self {
        Self { 
            size: num_vectors,
            vectors: result_vectors, 
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct SearchResult {
   pub index: usize,
   pub distance: u8,
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for SearchResult {}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
        .then_with(|| self.index.cmp(&other.index))
    }
}

impl SearchResult {
    pub fn new(index: &usize, distance: &u8) -> Self {
        SearchResult { 
            index: *index, 
            distance: *distance,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MergeResult {
   pub result: SearchResult,
   pub shard_index: usize,
   pub result_index: usize,
}

impl PartialEq for MergeResult {
    fn eq(&self, other: &Self) -> bool {
        self.result.distance == other.result.distance
    }
}

impl Eq for MergeResult {}

impl PartialOrd for MergeResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MergeResult {
    fn cmp(&self, other: &Self) -> Ordering {
        other.result.distance.cmp(&self.result.distance)
    }
}

impl MergeResult {
    pub fn new(result: SearchResult, shard_idx: usize) -> Self {
        Self { 
            result, 
            shard_index: shard_idx,
            result_index: 0
        }
    }
}