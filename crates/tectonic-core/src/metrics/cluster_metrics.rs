// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{fmt::Debug};

use crate::{error::TectonicError, utility::typings::usize_to_f32};

// ============================================================
// INTERNAL CLUSTER METRICS
// ============================================================
// Lightweight cluster-based metrics used for routing, observability, and
// eviction candidate ranking.
// ---
// `ClusterMetrics` intentionally stores only O(1)-update counters for efficiency.
// It does not retain per-entry history. This keeps mutation cheap in cache
// hot path while still allowing the repository to rank partitions by relative
// eviction weakness.
// ---
// Semantics:
// - `count`: number of live entries in the partition.
// - `bytes`: estimated live memory owned by entries in the partition.
// - `hits`: lifetime hit count.
// - `inserts`: lifetime insert count.
// - `evictions`: lifetime eviction count.
// - `last_access_tick`: logical timestamp of the most recent hit/insert/evict.
// - `distance_sum`: sum of live entry distances to the partition centroid.
// ---
// Invariants:
// - `count == 0` implies `bytes == 0`.
// - `count == 0` implies `distance_sum == 0.0`.
// - `distance_sum` must be finite.
// - `evictions <= inserts` under normal cache operation.

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct ClusterMetrics {
    count: usize,
    bytes: usize,
    hits: usize,
    inserts: usize,
    evictions: usize,
    last_access_tick: u64,
    distance_sum: f32,
}

// ============================================================
// CLUSTER METRICS: CONSTRUCTORS
// ============================================================

impl Debug for ClusterMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusterMetric")
            .field("count", &self.count)
            .field("bytes", &self.bytes)
            .field("entry_hits", &self.hits)
            .field("inserts", &self.inserts)
            .field("evictions", &self.evictions)
            .field("last_access_tick", &self.last_access_tick)
            .field("distance_sum", &self.distance_sum)
            .field("mean_distance_to_centroid", &self.mean_distance_to_centroid())
            .field("churn_rate", &self.churn_rate())
            .field("hits_pr_byte", &self.hits_pr_byte())
            .finish()
    }
}

impl Default for ClusterMetrics {
    fn default() -> Self {
        Self {
            count: 0,
            bytes: 0,
            hits: 0,
            inserts: 0,
            evictions: 0,
            last_access_tick: 0,
            distance_sum: 0.0 
        }
    }
}

// ============================================================
// CLUSTER METRICS: METHODS
// ============================================================

#[allow(dead_code)]
impl ClusterMetrics {
    #[inline]
    pub fn on_hit(&mut self, tick: u64) {
        self.hits = self.hits.saturating_add(1);
        self.last_access_tick = tick;
    }

    #[inline]
    pub fn on_insert(&mut self, bytes: usize, distance: f32, tick: u64)  {
        debug_assert!(distance.is_finite(), "Insert Distance-value must be finite");
        self.count = self.count.saturating_add(1);
        self.bytes = self.bytes.saturating_add(bytes);
        self.inserts = self.inserts.saturating_add(1);
        self.last_access_tick = tick;
    
        if distance.is_finite() {
            self.distance_sum += distance.max(0.0);
        }

        #[cfg(debug_assertions)]
        self.debug_assertion_validate();
    }

    #[inline]
    pub fn on_evict(&mut self, bytes: usize, distance: f32, tick: u64)  {
        debug_assert!(self.count > 0, "Eviction count must be positive before eviction");
        debug_assert!(self.bytes >= bytes, "Eviction bytes must not exceed current byte count");
        debug_assert!(distance.is_finite(), "Distance-value must be finite");

        self.count = self.count.saturating_sub(1);
        self.bytes = self.bytes.saturating_sub(bytes);
        self.inserts = self.inserts.saturating_sub(1);
        self.last_access_tick = tick;

        if self.count == 0 {
            self.bytes = 0;
            self.distance_sum = 0.0
        } else if distance.is_finite() {
            self.distance_sum = (self.distance_sum - distance.max(0.0)).max(0.0);
        }

        #[cfg(debug_assertions)]
        self.debug_assertion_validate();
    }

// ============================================================
// CLUSTER METRICS: DEBUGGING
// ============================================================

#[inline]
pub fn validate(&self) -> Result<(), TectonicError> {
    if !self.distance_sum.is_finite() {
        return Err(TectonicError::repository("ClusterMetrics distance_sum must be finite!"));
    }

    if self.count == 0 && self.bytes != 0 {
        return Err(TectonicError::repository("ClusterMetrics with zero count must have zero bytes!"));
    }

    if self.count == 0 && self.distance_sum != 0.0 {
        return Err(TectonicError::repository("ClusterMetrics with zero count must have zero distance_sum!"));
    }

    if self.evictions > self.inserts {
        return Err(TectonicError::repository("ClusterMetrics evictions cannot exceed inserts!"));
    }
    Ok(())
}

#[inline]
#[cfg(debug_assertions)]
pub fn debug_assertion_validate(&self) {
    debug_assert!(self.distance_sum.is_finite());
    debug_assert!(self.count > 0 || self.bytes == 0);
    debug_assert!(self.count > 0 || self.distance_sum == 0.0);
    debug_assert!(self.evictions <= self.inserts);
}

// ============================================================
// CLUSTER METRICS: HELPER-METHODS
// ============================================================

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    #[inline]
    pub fn entry_hits(&self) -> usize {
        self.hits
    }

    #[inline]
    pub fn inserts(&self) -> usize {
        self.inserts
    }

    #[inline]
    pub fn evictions(&self) -> usize {
        self.evictions
    }

    #[inline]
    pub fn last_access_tick(&self) -> u64 {
        self.last_access_tick
    }

    #[inline]
    pub fn distance_sum(&self) -> f32 {
        self.distance_sum
    }

// ============================================================
// CLUSTER METRICS: SCORE METHODS
// ============================================================

    #[inline]
    pub fn mean_distance_to_centroid(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.distance_sum / usize_to_f32(self.count)
        }
    }

    #[inline]
    pub fn churn_rate(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            usize_to_f32(self.inserts.saturating_add(self.evictions)) / usize_to_f32(self.count)
        }
    }

    #[inline]
    pub fn hits_pr_byte(&self) -> f32 {
        if self.bytes == 0 {
            0.0
        } else {
            usize_to_f32(self.hits) / usize_to_f32(self.bytes)
        }
    }

    #[inline]
    pub fn occupancy_pressure(&self, target: usize) -> f32 {
        if target == 0 {
            0.0
        } else {
            usize_to_f32(self.bytes) / usize_to_f32(target)
        }
    }

    #[inline]
    pub fn age(&self, current_tick: u64) -> u64 {
        current_tick.saturating_sub(self.last_access_tick)
    }

    // Computes a cheap partition-level weakness score for entry eviction.
    // ---
    // Higher score means the partition is a stronger eviction candidate.
    // Lower score indiciates a weak partition => Best for eviction
    // ---
    // Inputs:
    // - `current_tick`: global logical clock.
    // - `target_bytes`: desired byte capacity for the partition.
    //
    // Score components:
    // - high age increases weakness;
    // - high churn increases weakness;
    // - high mean centroid distance increases weakness;
    // - high occupancy pressure increases weakness;
    // - high hit density reduces weakness.
    #[inline]
    pub fn weakness_score(&self, current_tick: u64, target_bytes: usize) -> f32 {
        if self.count == 0 {
            return f32::NEG_INFINITY;
        }

        let age = self.age(current_tick) as f32;
        let churn = self.churn_rate();
        let distance = self.mean_distance_to_centroid();
        let pressure = self.occupancy_pressure(target_bytes);
        let hits_pr_byte = self.hits_pr_byte();

        let age_score = age.ln_1p();
        let penalty = hits_pr_byte.ln_1p();

        age_score 
            + churn 
            + distance 
            + pressure 
            - penalty
    }
}