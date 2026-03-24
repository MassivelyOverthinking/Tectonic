// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::fmt::Debug;

use crate::utility::typings::usize_to_f32;

// ============================================================
// INTERNAL VECTOR METRICS
// ============================================================

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct EntryMetrics {
    bytes: usize,
    hit_count: usize,
    insert_tick: u64,
    last_access_tick: u64,
    distance_to_centroid: f32,
}

impl Debug for EntryMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntryMetrics")
            .field("bytes", &self.bytes)
            .field("hit_count", &self.hit_count)
            .field("insert_tick", &self.insert_tick)
            .field("last_access_tick", &self.last_access_tick)
            .field("distance_to_centroid", &self.distance_to_centroid)
            .finish()
    }
}

impl Default for EntryMetrics {
    fn default() -> Self {
        Self {
            bytes: 0,
            hit_count: 0,
            insert_tick: 0,
            last_access_tick: 0,
            distance_to_centroid: 0.0 
        }
    }
}

impl EntryMetrics {

// ============================================================
// INTERNAL CONSTRUCTOR
// ============================================================

    #[inline]
    pub fn new(bytes: usize, distance: f32, tick: u64) -> Self {
        Self {
            bytes,
            hit_count: 0,
            insert_tick: tick,
            last_access_tick: tick,
            distance_to_centroid: distance
        }
    }

// ============================================================
// INTERNAL METRICS UPDATES
// ============================================================

    #[inline]
    pub fn on_hit(&mut self, tick: u64) {
        self.hit_count += 1;
        self.last_access_tick = tick;
    }

    #[inline]
    pub fn on_update_bytes(&mut self, bytes: usize) {
        self.bytes = bytes;
    }

    #[inline]
    pub fn on_reassign(&mut self, distance: f32) {
        self.distance_to_centroid = distance;
    }

// ============================================================
// ENTRY ACCESS METHODS
// ============================================================

    #[inline]
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    #[inline]
    pub fn hit_count(&self) -> usize {
        self.hit_count
    }

    #[inline]
    pub fn insert_tick(&self) -> u64 {
        self.insert_tick
    }

    #[inline]
    pub fn last_access_tick(&self) -> u64 {
        self.last_access_tick
    }

    #[inline]
    pub fn distance_to_centroid(&self) -> f32 {
        self.distance_to_centroid
    }
    
// ============================================================
// ENTRY METRICS METHODS
// ============================================================

    #[inline]
    pub fn time_since_access(&self, current_tick: u64) -> u64 {
        current_tick.saturating_sub(self.last_access_tick)
    }

    #[inline]
    pub fn time_since_insert(&self, current_tick: u64) -> u64 {
        current_tick.saturating_sub(self.insert_tick)
    }

    #[inline]
    pub fn hits_pr_byte(&self) -> f32 {
        if self.bytes == 0 {
            0.0
        } else {
            usize_to_f32(self.hit_count) / usize_to_f32(self.bytes)
        }
    }

    #[inline]
    pub fn was_nver_hit(&self) -> bool {
        self.hit_count == 0
    }

    #[inline]
    pub fn is_cold_since_insert(&self, current_tick: u64) -> bool {
        self.hit_count == 0 && current_tick > self.insert_tick
    }
}