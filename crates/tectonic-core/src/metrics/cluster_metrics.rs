// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{fmt::Debug};

use crate::utility::typings::{usize_to_f32};

// ============================================================
// INTERNAL PARTITION METRICS
// ============================================================

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
// CLUSTER METRICS UPDATES
// ============================================================

#[allow(dead_code)]
impl ClusterMetrics {
    
    #[inline]
    pub fn on_hit(&mut self, tick: u64) {
        self.hits += 1;
        self.last_access_tick = tick;
    }

    #[inline]
    pub fn on_insert(&mut self, bytes: usize, distance: f32, tick: u64)  {
        self.count += 1;
        self.bytes += bytes;
        self.inserts += 1;
        self.last_access_tick = tick;
        self.distance_sum += distance;
    }

    #[inline]
    pub fn on_evict(&mut self, bytes: usize, distance: f32, tick: u64)  {
        debug_assert!(self.count > 0);
        debug_assert!(self.bytes >= bytes);
        debug_assert!(self.distance_sum >= distance || self.count == 1);

        self.count -= 1;
        self.bytes -= bytes;
        self.inserts -= 1;
        self.last_access_tick = tick;

        if self.count == 0 {
            self.distance_sum = 0.0
        } else {
            self.distance_sum -= distance;
        }
    }

// ============================================================
// CLUSTER ACCESS METHODS
// ============================================================

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
// CLUSTER METRICS METHODS
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
            usize_to_f32(self.inserts + self.evictions) / usize_to_f32(self.count)
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
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

}