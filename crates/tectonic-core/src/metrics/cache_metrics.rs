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
    // CACHE METRICS: MAIN METHODS
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
        self.debug_assertion_valid();
    }

    #[inline]
    pub fn on_remove(&mut self, latency: Duration) {
        debug_assert!(self.size > 0, "Cannot remove entry from Empty Cache");

        self.size = self.size.saturating_sub(1);
        self.actions.record_remove(latency);

        #[cfg(debug_assertions)]
        self.debug_assertion_valid();
    }

    #[inline]
    pub fn on_evict(&mut self, latency: Duration) {
        debug_assert!(self.size > 0, "Cannot remove entry from Empty Cache");

        self.size = self.size.saturating_sub(1);
        self.actions.record_evict(latency);

        #[cfg(debug_assertions)]
        self.debug_assertion_valid();
    }

    #[inline]
    pub fn on_update(&mut self, latency: Duration) {
        self.actions.record_update(latency);

        #[cfg(debug_assertions)]
        self.debug_assertion_valid();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.created_at = Instant::now();
        self.size = 0;
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
        self.actions.record_search(latency, requested_k, returned_k, scanned_k);

        #[cfg(debug_assertions)]
        self.debug_assertion_valid();
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

    #[inline]
    pub fn validate(&self) -> TectonicResult<()> {
        if self.size > self.capacity {
            return Err(TectonicError::inconsistent_state(
                "Cache metrics size exceed capacity",
            ));
        };

        self.actions.validate()?;
        Ok(())
    }

    #[inline]
    pub fn debug_assertion_valid(&self) {
        debug_assert!(self.size <= self.capacity);
        self.actions.debug_assertion_valid();
    }

    // ============================================================
    // CACHE METRICS: ACCESSORS
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
    // CACHE METRICS: HELPER METHODS
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
// ACTION METRICS: CONSTRUCTOR
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ActionMetrics {
    total_actions: usize,

    insert_actions: usize,
    remove_actions: usize,
    search_actions: usize,
    update_actions: usize,
    eviction_actions: usize,

    requested_results: usize,
    returned_results: usize,
    scanned_results: usize,

    total_latency_ns: u128,
    insert_latency_ns: u128,
    remove_latency_ns: u128,
    search_latency_ns: u128,
    update_latency_ns: u128,
    eviction_latency_ns: u128,
}

impl Default for ActionMetrics {
    #[inline]
    fn default() -> Self {
        Self {
            total_actions: 0,

            insert_actions: 0,
            remove_actions: 0,
            search_actions: 0,
            update_actions: 0,
            eviction_actions: 0,
            
            requested_results: 0,
            returned_results: 0,
            scanned_results: 0,

            total_latency_ns: 0,
            insert_latency_ns: 0,
            remove_latency_ns: 0,
            search_latency_ns: 0,
            update_latency_ns: 0,
            eviction_latency_ns: 0
        }
    }
}

// ============================================================
// ACTION METRICS: MAIN METHODS
// ============================================================

#[allow(dead_code)]
impl ActionMetrics {
    #[inline]
    pub fn record_insert(&mut self, latency: Duration) {
        // Convert Duration-instance to Nanoseconds.
        let ns = latency.as_nanos();

        // Add the attributes to correct internal values.
        self.total_actions = self.total_actions.saturating_add(1);
        self.insert_actions = self.insert_actions.saturating_add(1);
        self.total_latency_ns = self.total_latency_ns.saturating_add(ns);
        self.insert_latency_ns = self.insert_latency_ns.saturating_add(ns);
    }

    #[inline]
    pub fn record_remove(&mut self, latency: Duration) {
        // Convert Duration-instance to Nanoseconds.
        let ns = latency.as_nanos();

        // Add the attributes to correct internal values.
        self.total_actions = self.total_actions.saturating_add(1);
        self.remove_actions = self.remove_actions.saturating_add(1);
        self.total_latency_ns = self.total_latency_ns.saturating_add(ns);
        self.remove_latency_ns = self.remove_latency_ns.saturating_add(ns);
    }

    #[inline]
    pub fn record_update(&mut self, latency: Duration) {
        // Convert Duration-instance to Nanoseconds.
        let ns = latency.as_nanos();

        // Add the attributes to correct internal values.
        self.total_actions = self.total_actions.saturating_add(1);
        self.update_actions = self.update_actions.saturating_add(1);
        self.total_latency_ns = self.total_latency_ns.saturating_add(ns);
        self.update_latency_ns = self.update_latency_ns.saturating_add(ns);
    }

    #[inline]
    pub fn record_evict(&mut self, latency: Duration) {
        // Convert Duration-instance to Nanoseconds.
        let ns = latency.as_nanos();

        // Add the attributes to correct internal values.
        self.total_actions = self.total_actions.saturating_add(1);
        self.eviction_actions = self.eviction_actions.saturating_add(1);
        self.total_latency_ns = self.total_latency_ns.saturating_add(ns);
        self.eviction_latency_ns = self.eviction_latency_ns.saturating_add(ns);
    }

    #[inline]
    pub fn record_search(
        &mut self, 
        latency: Duration, 
        requested: usize, 
        returned: usize, 
        scanned: usize, 
    ) {
        debug_assert!(
            returned <= requested || requested == 0,
            "Returned more search results that requested"
        );

        let ns = latency.as_nanos();

        self.total_actions = self.total_actions.saturating_add(1);
        self.search_actions = self.search_actions.saturating_add(1);

        self.requested_results = self.requested_results.saturating_add(requested);
        self.returned_results = self.returned_results.saturating_add(returned);
        self.scanned_results = self.scanned_results.saturating_add(scanned);

        self.total_latency_ns = self.total_latency_ns.saturating_add(ns);
        self.search_latency_ns = self.search_latency_ns.saturating_add(ns);
    }

    // ============================================================
    // ACTION METRICS: VALIDATION
    // ============================================================

    #[inline]
    pub fn validate(&self) -> TectonicResult<()> {
        let grouped_actions = self
            .insert_actions
            .saturating_add(self.remove_actions)
            .saturating_add(self.search_actions)
            .saturating_add(self.update_actions)
            .saturating_add(self.eviction_actions);

        if grouped_actions != self.total_actions {
            return Err(TectonicError::inconsistent_state(
                "Total Action metrics does not equal grouped actions!"
            ));
        };

        if self.returned_results > self.requested_results {
            return Err(TectonicError::inconsistent_state(
                "Returned search results exceed requested search results"
            ));
        };

        Ok(())
    }

    #[inline]
    pub fn debug_assertion_valid(&self) {
        let grouped_actions = self
            .insert_actions
            .saturating_add(self.remove_actions)
            .saturating_add(self.search_actions)
            .saturating_add(self.update_actions)
            .saturating_add(self.eviction_actions);

        debug_assert_eq!(grouped_actions, self.total_actions);
        debug_assert!(self.returned_results <= self.requested_results);
    }

    // ============================================================
    // ACTION METRICS: ACCESSORS
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
    pub fn search_actions(&self) -> usize {
        self.search_actions
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
    pub fn requested_results(&self) -> usize {
        self.requested_results
    }

    #[inline]
    pub fn returned_results(&self) -> usize {
        self.returned_results
    }

    #[inline]
    pub fn scanned_results(&self) -> usize {
        self.scanned_results
    }

    // ============================================================
    // ACTION METRICS: SEARCH RESULTS
    // ============================================================

    #[inline]
    pub fn search_selectivity(&self) -> f32 {
        if self.search_actions == 0 {
            0.0
        } else {
            usize_to_f32(self.returned_results) / usize_to_f32(self.requested_results)
        }
    }

    #[inline]
    pub fn average_results_per_search(&self) -> f32 {
        if self.search_actions == 0 {
            0.0
        } else {
            usize_to_f32(self.returned_results) / usize_to_f32(self.search_actions)
        }
    }

    #[inline]
    pub fn average_candidates_per_search(&self) -> f32 {
        if self.search_actions == 0 {
            0.0
        } else {
            usize_to_f32(self.scanned_results) / usize_to_f32(self.search_actions)
        }
    }

    // ============================================================
    // ACTION METRICS: LATENCY
    // ============================================================

    #[inline]
    fn average_ns(latency: u128, count: usize) -> f64 {
        if count == 0 {
            0.0
        } else {
            latency as f64 / count as f64
        }
    }

    #[inline]
    pub fn average_latency_ns(&self) -> f64 {
        Self::average_ns(self.total_latency_ns, self.total_actions)
    }

    #[inline]
    pub fn average_insert_latency_ns(&self) -> f64 {
        Self::average_ns(self.insert_latency_ns, self.total_actions)
    }

    #[inline]
    pub fn average_remove_latency_ns(&self) -> f64 {
        Self::average_ns(self.remove_latency_ns, self.total_actions)
    }

    #[inline]
    pub fn average_search_latency_ns(&self) -> f64 {
        Self::average_ns(self.search_latency_ns, self.total_actions)
    }

    #[inline]
    pub fn average_update_latency_ns(&self) -> f64 {
        Self::average_ns(self.update_latency_ns, self.total_actions)
    }

    #[inline]
    pub fn average_eviction_latency_ns(&self) -> f64 {
        Self::average_ns(self.eviction_latency_ns, self.total_actions)
    }
}
