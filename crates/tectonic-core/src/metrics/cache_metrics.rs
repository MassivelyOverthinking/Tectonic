// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::{fmt::Debug, time::{Duration, Instant}};

use crate::{error::TectonicError, utility::typings::{TectonicResult, usize_to_f32}};

// ============================================================
// INTERNAL CACHE METRICS
// ============================================================
// Main cache-level metrics.
// ---
// Metrics track cache occupancy stats, operation counts, search behavior,
// latency, and cache lifecycle. 
// Unlike standard key-value cache elements, vector-based similarity search
// does not possess a clean "miss" concept. 
// A query may return fewer than desired (K)
// results, no results at all, or approximate candidates, but this is not equivalent
// to a lookup miss.
//
// Therefore cache metrics tracks:
// - insert/remove/update/eviction operations.
// - search/query operations.
// - number of returned search results.
// - number of candidate vectors inspected.
// - latency per operation class.
// - cache occupancy and throughput.

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct CacheMetrics {
    created_at: Instant,
    size: usize,
    capacity: usize,
    actions: ActionMetrics,
}

impl Debug for CacheMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache Metrics")
            .field("size", &self.size)
            .field("capacity", &self.capacity)
            .field("load_factor", &self.load_factor())
            .field("free_capacity", &self.free_capacity())
            .field("is_full", &self.is_full())
            .field("is_empty", &self.is_empty())
            .field("uptime", &self.uptime())
            .field("operations_per_sec", &self.operations_per_second())
            .field("actions", &self.actions)
            .finish()
    }
}

#[allow(dead_code)]
impl CacheMetrics {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            created_at: Instant::now(),
            size: 0,
            capacity,
            actions: ActionMetrics::default()
        }
    }

// ============================================================
// CACHE STATE EVENTS
// ============================================================

    #[inline]
    pub fn on_insert(&mut self, latency: Duration) {
        debug_assert!(
            self.size < self.capacity,
            "Insert element would exceed current cache size"
        );

        self.size = self.size.saturating_add(1);
        self.actions.record_insert(latency);

        #[cfg(debug_assertions)]
        todo!()
    }

    #[inline]
    pub fn on_remove(&mut self, latency: Duration) {
        debug_assert!(self.size > 0, "Cannot remove entry from Empty Cache");

        self.size = self.size.saturating_sub(1);
        self.actions.record_remove(latency);

        #[cfg(debug_assertions)]
        todo!()
    }

    #[inline]
    pub fn on_evict(&mut self, latency: Duration) {
        debug_assert!(self.size > 0, "Cannot remove entry from Empty Cache");

        self.size = self.size.saturating_sub(1);
        self.actions.record_evict(latency);

        #[cfg(debug_assertions)]
        todo!()
    }

    #[inline]
    pub fn on_update(&mut self, latency: Duration) {
        self.actions.record_update(latency);

        #[cfg(debug_assertions)]
        todo!()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.created_at = Instant::now();
        self.size = 0;
        self.capacity = 0;
        self.actions = ActionMetrics::default();
    }

    #[inline]
    pub fn on_search(
        &mut self,
        latency: Duration,
        requested_k: usize,
        returned_k: usize,
        scanned_k: usize
    ) {
        todo!()
    }

    #[inline]
    pub fn set_size(&mut self, size: usize) -> TectonicResult<()> {
        if size > self.capacity {
            return Err(TectonicError::invalid_parameter(
                "Cache metrics size", 
                "Msust not exceed capacity"
            ));
        };

        self.size = size;
        Ok(())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.created_at = Instant::now();
        self.size = 0;
        self.actions = ActionMetrics::default();
    }

    // ============================================================
    // CACHE METRICS: VALIDATION
    // ============================================================



    // ============================================================
    // CACHE BASIC ACCESSORS
    // ============================================================

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn actions(&self) -> &ActionMetrics {
        &self.actions
    }

    #[inline]
    pub fn uptime(&self) -> Duration {
        self.created_at.elapsed()
    }

// ============================================================
// CACHE DERIVED METRICS
// ============================================================

    #[inline]
    pub fn load_factor(&self) -> f32 {
        if self.capacity == 0 {
            0.0
        } else {
            usize_to_f32(self.size) / usize_to_f32(self.capacity)
        }
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn free_capacity(&self) -> usize {
        self.capacity.saturating_sub(self.size)
    }

    #[inline]
    pub fn operations_per_second(&self) -> f64 {
        let elapsed = self.uptime().as_secs_f64();
        if elapsed == 0.0 {
            0.0
        } else {
            self.actions.total_actions() as f64 / elapsed
        }
    }

    #[inline]
    pub fn search_selectivity(&self) {}

    #[inline]
    pub fn average_results_pr_search(&self) {}

    #[inline]
    pub fn average_candidates_pr_search(&self) {}
}

// ============================================================
// INTERNAL ACTION METRICS
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ActionMetrics {
    total_actions: usize,

    insert_actions: usize,
    remove_actions: usize,
    get_actions: usize,
    update_actions: usize,
    eviction_actions: usize,

    get_hits: usize,
    get_misses: usize,

    total_latency_ns: u128,
    insert_latency_ns: u128,
    remove_latency_ns: u128,
    get_latency_ns: u128,
    update_latency_ns: u128,
    eviction_latency_ns: u128,
}

impl Default for ActionMetrics {
    fn default() -> Self {
        Self {
            total_actions: 0,
            insert_actions: 0,
            remove_actions: 0,
            get_actions: 0,
            update_actions: 0,
            eviction_actions: 0,
            get_hits: 0,
            get_misses: 0,
            total_latency_ns: 0,
            insert_latency_ns: 0,
            remove_latency_ns: 0,
            get_latency_ns: 0,
            update_latency_ns: 0,
            eviction_latency_ns: 0
        }
    }
}

// ============================================================
// INTERNAL ACTION METHODS
// ============================================================

#[allow(dead_code)]
impl ActionMetrics {
    #[inline]
    pub fn record_insert(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.insert_actions += 1;
        self.total_latency_ns += ns;
        self.insert_latency_ns += ns;
    }

    #[inline]
    pub fn record_remove(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.remove_actions += 1;
        self.total_latency_ns += ns;
        self.remove_latency_ns += ns;
    }

    #[inline]
    pub fn record_get_hit(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.get_actions += 1;
        self.get_hits += 1;
        self.total_latency_ns += ns;
        self.get_latency_ns += ns;
    }

    #[inline]
    pub fn record_get_miss(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.get_actions += 1;
        self.get_misses += 1;
        self.total_latency_ns += ns;
        self.get_latency_ns += ns;
    }

    #[inline]
    pub fn record_update(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.update_actions += 1;
        self.total_latency_ns += ns;
        self.update_latency_ns += ns;
    }

    #[inline]
    pub fn record_evict(&mut self, latency: Duration) {
        let ns = latency.as_nanos();
        self.total_actions += 1;
        self.eviction_actions += 1;
        self.total_latency_ns += ns;
        self.eviction_latency_ns += ns;
    }

// ============================================================
// ACTION BASIC ACCESSORS
// ============================================================

    #[inline]
    pub fn total_actions(&self) -> usize {
        self.total_actions
    }

    #[inline]
    pub fn insert_actions(&self) -> usize {
        self.insert_actions
    }

    #[inline]
    pub fn remove_actions(&self) -> usize {
        self.remove_actions
    }

    #[inline]
    pub fn get_actions(&self) -> usize {
        self.get_actions
    }

    #[inline]
    pub fn update_actions(&self) -> usize {
        self.update_actions
    }

    #[inline]
    pub fn eviction_actions(&self) -> usize {
        self.eviction_actions
    }

    #[inline]
    pub fn get_hits(&self) -> usize {
        self.get_hits
    }

    #[inline]
    pub fn get_misses(&self) -> usize {
        self.get_misses
    }

// ============================================================
// ACTION DERIVED METRICS
// ============================================================

    #[inline]
    pub fn hit_rate(&self) -> f32 {
        let total_gets = self.get_hits + self.get_misses;
        if total_gets == 0 {
            0.0
        } else {
            self.get_hits as f32 / total_gets as f32
        }
    }

    #[inline]
    pub fn miss_rate(&self) -> f32 {
        let total_gets = self.get_hits + self.get_misses;
        if total_gets == 0 {
            0.0
        } else {
            self.get_misses as f32 / total_gets as f32
        }
    }

    #[inline]
    pub fn average_latency_ns(&self) -> f64 {
        if self.total_actions == 0 {
            0.0
        } else {
            self.total_latency_ns as f64 / self.total_actions as f64
        }
    }

    #[inline]
    pub fn average_insert_latency_ns(&self) -> f64 {
        if self.insert_actions == 0 {
            0.0
        } else {
            self.insert_latency_ns as f64 / self.insert_actions as f64
        }
    }

    #[inline]
    pub fn average_remove_latency_ns(&self) -> f64 {
        if self.remove_actions == 0 {
            0.0
        } else {
            self.remove_latency_ns as f64 / self.remove_actions as f64
        }
    }

    #[inline]
    pub fn average_get_latency_ns(&self) -> f64 {
        if self.get_actions == 0 {
            0.0
        } else {
            self.get_latency_ns as f64 / self.get_actions as f64
        }
    }

    #[inline]
    pub fn average_update_latency_ns(&self) -> f64 {
        if self.update_actions == 0 {
            0.0
        } else {
            self.update_latency_ns as f64 / self.update_actions as f64
        }
    }

    #[inline]
    pub fn average_eviction_latency_ns(&self) -> f64 {
        if self.eviction_actions == 0 {
            0.0
        } else {
            self.eviction_latency_ns as f64 / self.eviction_actions as f64
        }
    }

}
