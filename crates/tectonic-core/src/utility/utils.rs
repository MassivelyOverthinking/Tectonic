// ============================================================
// IMPORTS AND MODULES
// ============================================================

use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::admission::admission_strategy::{Admission, AdmissionStrategy};
use crate::admission::always_admission::AlwaysAdmission;
use crate::admission::tinylfu_admission::TinyLFUAdmission;
use crate::admission::twohit_admission::TwoHitAdmission;
use crate::admission::windowlfu_admission::WindowTinyLFUAdmisssion;
use crate::error::TectonicError;
use crate::eviction::eviction_strategy::{Eviction, EvictionStrategy};
use crate::eviction::partitioned_fifo::PartitionedFIFO;
use crate::eviction::partitioned_lifo::PartitionedLIFO;
use crate::eviction::partitioned_lru::PartitionedLRU;
use crate::eviction::segmented_lru::SegmentedLRU;
use crate::eviction::varc::VARC;
use crate::location::location_entry::{ShardEntry};
use crate::utility::typings::DimVector;

// ============================================================
// GENERAL UTILITY METHODS & STRUCTS
// ============================================================

#[derive(Debug, Clone, Copy, Eq, Hash)]
#[allow(dead_code)]
pub struct UniqueID {
    pub slot_id: usize,
    pub gen_id: u32,
}

impl Display for UniqueID  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{})", self.gen_id, self.slot_id)
    }
}

impl PartialEq for UniqueID {
    fn eq(&self, other: &Self) -> bool {
        self.slot_id == other.slot_id && self.gen_id == other.gen_id
    }
}

impl Ord for UniqueID {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.slot_id.cmp(&other.slot_id) {
            Ordering::Equal => self.gen_id.cmp(&other.gen_id),
            other => other,
        }
    }
}

impl PartialOrd for UniqueID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
impl UniqueID {
    pub fn new(slot: usize, generation: u32) -> Self {
        Self {
            slot_id: slot,
            gen_id: generation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    admission_policy: Admission,
    eviction_policy: Eviction,
    capacity: usize,

    // Admission Parameters
    threshold: f32,
    tiny_lfu_width: usize,
    tiny_lfu_depth: usize,
    tiny_lfu_frequency: u8,
    window_capacity: usize,
}

impl StrategyConfig {
    #[inline]
    pub fn new(admission_policy: Admission, eviction_policy: Eviction, capacity: usize) -> Self {
        Self { 
            admission_policy, 
            eviction_policy, 
            capacity, 
            threshold: 0.0, 
            tiny_lfu_width: 4096, 
            tiny_lfu_depth: 4, 
            tiny_lfu_frequency: 2, 
            window_capacity: 256, 
        }
    }

    #[inline]
    fn debug_assertions_state(&self) {
        debug_assert!(self.capacity > 0, "Strategy Config capacity must exceed 0");
        debug_assert!(self.threshold.is_finite(), "Strategy Config threshold must be initie");
        debug_assert!(self.tiny_lfu_width > 0, "Strategy Config widtd must exceed 0");
        debug_assert!(self.tiny_lfu_depth > 0, "Strategy Config depth must exceed 0");
        debug_assert!(self.tiny_lfu_frequency > 0, "Strategy Config frequncy must exceed 0");
        debug_assert!(self.window_capacity > 0, "Strategy Config window capacity must exceed 0");
    }
}

pub struct  StrategyStructure {
    admission: Box<dyn AdmissionStrategy>,
    eviction: Box<dyn EvictionStrategy>,
}

impl StrategyStructure {
    #[inline]
    pub fn new(admission: Box<dyn AdmissionStrategy>, eviction: Box<dyn EvictionStrategy>) -> Self {
        Self { 
            admission, 
            eviction 
        }
    }

    #[inline]
    pub fn from_config(config: StrategyConfig) -> Self {
        #[cfg(debug_assertions)]
        config.debug_assertions_state();

        Self { 
            admission: build_admission_strategy(&config), 
            eviction: build_eviction_strategy(&config), 
        }
    }

    #[inline]
    pub fn get_admission(&self) -> &dyn AdmissionStrategy {
        self.admission.as_ref()
    }

    #[inline]
    pub fn get_admission_mut(&mut self) -> &mut dyn AdmissionStrategy {
        self.admission.as_mut()
    }

    #[inline]
    pub fn get_eviction(&self) -> &dyn EvictionStrategy {
        self.eviction.as_ref()
    }

    #[inline]
    pub fn get_eviction_mut(&mut self) -> &mut dyn EvictionStrategy {
        self.eviction.as_mut()
    }
}

#[inline]
fn build_admission_strategy(config: &StrategyConfig) -> Box<dyn AdmissionStrategy> {
    match config.admission_policy {
        Admission::Always => {
            Box::new(AlwaysAdmission::new())
        },
        Admission::TwoHit => {
            Box::new(TwoHitAdmission::with_capacity(config.window_capacity))
        },
        Admission::TinyLFU => {
            Box::new(TinyLFUAdmission::with_params(
                config.tiny_lfu_width, 
                config.tiny_lfu_depth, 
                config.tiny_lfu_frequency
            ))
        },
        Admission::WeightedTinyLFU => {
            Box::new(WindowTinyLFUAdmisssion::with_params(
                config.tiny_lfu_width, 
                config.tiny_lfu_depth, 
                config.window_capacity, 
                config.tiny_lfu_frequency,
            ))
        }
    }
}

#[inline]
fn build_eviction_strategy(config: &StrategyConfig) -> Box<dyn EvictionStrategy> {
    match config.eviction_policy {
        Eviction::PartitionedLIFO => {
            Box::new(PartitionedLIFO::with_capacity(config.capacity))
        },
        Eviction::PartitionedFIFO => {
            Box::new(PartitionedFIFO::with_capacity(config.capacity, config.capacity))
        },
        Eviction::PartitionedLRU => {
            Box::new(PartitionedLRU::with_capacity(config.capacity))
        },
        Eviction::SegmentedLRU => {
            Box::new(SegmentedLRU::with_capacity(config.capacity))
        },
        Eviction::VARC => {
            Box::new(VARC::with_capacity(config.capacity))
        }
    }
}

pub fn calculate_sizes(max_entries: usize, elements: usize) -> Vec<usize> {
    let base_value = max_entries / elements;
    let remainder_value = max_entries % elements;

    let mut sizes = vec![base_value; elements];

    for size in &mut sizes[..remainder_value] {
        *size += 1;
    }

    sizes
}

#[allow(dead_code)]
pub fn hash_dimvector<const D: usize>(vector: &DimVector<D>) -> u64 {
    let mut vec_hash = DefaultHasher::new();
    for &value in vector.iter() {
        value.to_bits().hash(&mut vec_hash);
    }

    vec_hash.finish()
}

pub fn hash_shard_entry(entry: &ShardEntry) -> u64 {
    let mut hash_value = DefaultHasher::new();

    entry.get_id().hash(&mut hash_value);

    hash_value.finish()
}
 
#[allow(dead_code)]
pub fn secondary_arena_hash(hash: u64) -> u64 {
    hash.rotate_left(32) ^ 0x9e3779b97f4a7c15
}

pub fn validate_vector<const D: usize>(vector: &DimVector<D>) -> Result<(), TectonicError> {
    for index in 0..D {
        let value = vector[index];
        if !value.is_finite() {
            return Err(TectonicError::InvalidVectorError { 
                index: index 
            })
        }
    }
    Ok(())
}