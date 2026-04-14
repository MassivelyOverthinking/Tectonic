// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::time::Instant;

use crate::utility::utils::UniqueID;
use crate::utility::typings::VectorTier;

// ============================================================
// EVICTION ENTRY
// ============================================================

#[derive(Debug, Clone)]
pub struct EvictionEntry {
    entry_id: UniqueID,
    partition_id: usize,
    location: EvictionLocation,
    tier: VectorTier,
    created_at: Instant,
    data: EvictionData,
    scores: EvictionScores,
    victim_score: f64,
}

impl EvictionEntry {
    pub fn new(entry_id: UniqueID, partition_id: usize, location: EvictionLocation, tier: VectorTier) -> Self {
        Self {
            entry_id,
            partition_id,
            location,
            tier,
            created_at: Instant::now(),
            data: EvictionData::default(),
            scores: EvictionScores::default(),
            victim_score: 0.0,
        }
    }

    pub fn entry_id(&self) -> &UniqueID {
        &self.entry_id
    }

    pub fn partition_id(&self) -> &usize {
        &self.partition_id
    }

    pub fn location(&self) -> &EvictionLocation {
        &self.location
    }

    pub fn created_at(&self) -> &Instant {
        &self.created_at
    }

    pub fn tier(&self) -> &VectorTier {
        &self.tier
    }

    pub fn data(&self) -> &EvictionData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut EvictionData {
        &mut self.data
    }
}

#[derive(Debug, Clone)]
struct EvictionLocation {
    shard_idx: usize,
    slot_idx: usize,
    arena_idx: usize
}

impl EvictionLocation {
    pub fn new(shard_idx: usize, slot_idx: usize, arena_idx: usize) -> Self {
        Self {
            shard_idx,
            slot_idx,
            arena_idx
        }
    }

    pub fn get_shard_index(&self) -> &usize {
        &self.shard_idx
    }

    pub fn get_slot_index(&self) -> &usize {
        &self.slot_idx
    }
}

#[derive(Debug, Clone)]
struct EvictionData {
    average_distance: f64,
    access_count: usize,
    hit_count: usize,
    miss_count: usize,
    last_accessed: u64,
    score: f64,
}

impl Default for EvictionData {
    fn default() -> Self {
        Self {
            average_distance: 0.0,
            access_count: 0,
            hit_count: 0,
            miss_count: 0,
            last_accessed: 0,
            score: 0.0,
        }
    }
}

impl EvictionData {
    pub fn update_on_access(&mut self, distance: f64, timestamp: u64, is_hit: bool) {
        self.access_count += 1;
        if is_hit {
            self.hit_count += 1;
        } else {
            self.miss_count += 1;
        }
        self.last_accessed = timestamp;
        self.average_distance = self.update_average_distance(distance);
        self.score = self.calculate_score();
    }

    fn update_average_distance(&mut self, distance: f64) -> f64 {
        ((self.average_distance * (self.access_count as f64 - 1.0)) + distance) / (self.access_count as f64)
    }

    fn calculate_score(&self) -> f64 {
        // Placeholder for a more complex scoring algorithm
        self.average_distance * (self.miss_count as f64 / self.access_count as f64)
    }

    pub fn get_score(&self) -> f64 {
        self.score
    }
}

#[derive(Debug, Clone)]
struct EvictionScores {
    affinity_score: f64,
    contribution_score: f64,
    redundancy_score: f64,
}

impl Default for EvictionScores {
    fn default() -> Self {
        Self {
            affinity_score: 0.0,
            contribution_score: 0.0,
            redundancy_score: 0.0,
        }
    }
}

impl EvictionScores {
    pub fn get_affinity_score(&self) -> f64 {
        self.affinity_score
    }

    pub fn get_contribution_score(&self) -> f64 {
        self.contribution_score
    }

    pub fn get_redundancy_score(&self) -> f64 {
        self.redundancy_score
    }
}
