// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::time::Instant;

// ============================================================
// INTERNAL CACHE METRICS
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct CacheMetrics {
    created_at: Instant,
    size: usize,
    capacity: usize,
    load_factor: f32,
    actions: ActionMetrics,
}

impl CacheMetrics {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { 
            created_at: Instant::now(),
            size: 0, 
            capacity: capacity,
            load_factor: 0.0, 
            actions: ActionMetrics::default(),  
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn increment_size(&mut self) {
        self.size += 1;
        self.load_factor = self.size as f32 / self.capacity as f32;
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct ActionMetrics {
    total_actions: usize,
    insert_actions: usize,
    remove_actions: usize,
    get_actions: usize,
    average_latency: f32,
}

#[allow(dead_code)]
impl ActionMetrics {
    pub fn default() -> Self {
        Self { 
            total_actions: 0,
            insert_actions: 0,
            remove_actions: 0,
            get_actions: 0,
            average_latency: 0.0f32,
        }
    }

    pub fn update_average_latency(&mut self, latency: f32) {
        self.total_actions += 1;

        let num = self.total_actions as f32;
        self.average_latency += (latency - self.average_latency) / num;
    }

    pub fn update_insert_action(&mut self, latency: f32) {
        self.insert_actions += 1;
        self.update_average_latency(latency);
    }

    pub fn update_remove_action(&mut self, latency: f32) {
        self.remove_actions += 1;
        self.update_average_latency(latency);
    }

    pub fn update_get_action(&mut self, latency: f32) {
        self.get_actions += 1;
        self.update_average_latency(latency);
    }
}