// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::time::Instant;

use crate::utility::typings::{f32_to_usize, usize_to_f32};

// ============================================================
// INTERNAL PARTITION METRICS
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct ClusterMetrics {
    count: usize,
    bytes: usize,
    hits: usize,
    inserts: usize,
    evictions: usize,
    last_access: Instant,
    average_distance: f32,
}

impl ClusterMetrics  {
    pub fn default() -> Self {
        Self {
            count: 0,
            bytes: 0,
            hits: 0,
            inserts: 0,
            evictions: 0,
            last_access: Instant::now(),
            average_distance: 0.0,
        }
    }

    pub fn increment_metric_count(&mut self) {
        self.count += 1;
    }

    pub fn decrement_metric_count(&mut self) {
        self.count -= 1;
    }

    pub fn add_metric_bytes(&mut self, bytes: usize) {
        self.bytes + bytes;
    }

    pub fn remove_metric_bytes(&mut self, bytes: usize) {
        self.bytes - bytes;
    }

    pub fn increment_metric_hits(&mut self) {
        self.hits += 1;
    }

    pub fn decrement_metric_hits(&mut self) {
        self.hits -= 1;
    }

    pub fn increment_metric_inserts(&mut self) {
        self.inserts += 1;
    }

    pub fn decrement_metric_inserts(&mut self) {
        self.inserts -= 1;
    }

    pub fn increment_metric_evictions(&mut self) {
        self.evictions += 1;
    }

    pub fn decrement_metric_evictions(&mut self) {
        self.evictions -= 1;
    }

    pub fn update_metric_access(&mut self) {
        self.last_access = Instant::now();
    }

    pub fn add_metric_distance(&mut self, distance: f32) {
        let current_count = usize_to_f32(self.count);
        let new_average = self.average_distance * (current_count - 1.0) + distance / current_count;
        self.average_distance = new_average;
    }

    pub fn remove_metric_distance(&mut self, distance: f32) {
        if self.count == 0 {
            self.average_distance = 0.0;
            return;
        }

        let new_count = usize_to_f32(self.count);
        let old_count = new_count + 1.0;

        let new_average = (self.average_distance * old_count - distance) / new_count;
        self.average_distance = new_average;
    }

}