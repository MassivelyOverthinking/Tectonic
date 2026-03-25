// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{metrics::entry_metrics::EntryMetrics, utility::utils::UniqueID};
use crate::utility::typings::DimVector;
use core::fmt;
use std::cmp::Ordering;
use std::ops::Range;
use std::time::Duration;
use std::usize;

// ============================================================
// VECTOR ENTRY STRUCTURE
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VectorEntry<'a, const D: usize> {
    pub vector_id: UniqueID,
    pub user_id: Option<&'a str>,
    pub vector: DimVector<D>,
    pub metrics: Option<EntryMetrics>
}

impl<'a, const D: usize> VectorEntry<'a, D> {
    pub fn new(id: usize, generation: u32, user_id: Option<&'a str>, vector: DimVector<D>, metrics_enabled: bool) -> Self {
        Self { 
            vector_id: UniqueID::new(id, generation),
            user_id: user_id,
            vector,
            metrics: if metrics_enabled { Some(EntryMetrics::default()) } else { None },
        }
    }
}

// ============================================================
// CACHE RESULTS & ENTRY STRUCTURES
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacheEntry<const D: usize> {
   pub index: usize,
   pub vector: DimVector<D>,
   pub distance: f32,
}

impl<const D: usize> fmt::Display for CacheEntry<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CacheEntry(index: {}, distance: {:.6}, dims: {})",
            self.index,
            self.distance,
            D
        )
    }
}

impl<const D: usize> CacheEntry<D> {
    pub fn new(index: usize, vector: DimVector<D>, distance: f32) -> Self {
        Self {
            index,
            vector,
            distance
        }
    }
    
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacheResult<const D: usize> {
   pub k: usize,
   pub partitions: usize,
   pub candidates: usize,
   pub latency: Duration,
   pub entries: Vec<CacheEntry<D>>,
}

impl<const D: usize> CacheResult<D> {
    pub fn new(k: usize, partitions: usize, candidates: usize, latency: Duration, entries: Vec<CacheEntry<D>>) -> Self {
        Self { 
            k,
            partitions,
            candidates,
            latency,
            entries, 
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn best_distance(&self) -> Option<f32> {
        self.entries.first().map(|e| e.distance)
    }

    pub fn worst_distance(&self) -> Option<f32> {
        self.entries.last().map(|e| e.distance)
    }

    pub fn get(&self, index: usize) -> Option<&CacheEntry<D>> {
        self.entries.get(index)
    }

    pub fn first(&self) -> Option<&CacheEntry<D>> {
        self.entries.first()
    }

    pub fn last(&self) -> Option<&CacheEntry<D>> {
        self.entries.last()
    }

    pub fn as_slice(&self) -> &[CacheEntry<D>] {
        &self.entries
    }

    pub fn slice(&self, range: Range<usize>) -> &[CacheEntry<D>] {
        &self.entries[range]
    }

    pub fn top(&self, n: usize) -> &[CacheEntry<D>] {
        let end = n.min(self.entries.len());
        &self.entries[..end]
    }

    pub fn vectors(&self) -> Vec<&DimVector<D>> {
        self.entries.iter().map(|entry| &entry.vector).collect()
    }

    pub fn average_distance(&self) -> Option<f32> {
        if self.entries.is_empty() {
            return None;
        }

        let sum: f32 = self.entries.iter().map(|e| e.distance).sum();
        Some(sum / self.entries.len() as f32)
    }
}

// ============================================================
// SEARCH RESULT STRUCTURE
// ============================================================

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

// ============================================================
// MERGE RESULT STRUCTURE
// ============================================================

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