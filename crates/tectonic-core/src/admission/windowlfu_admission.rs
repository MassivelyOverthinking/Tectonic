// ============================================================
// IMPORTS AND MODULES
// ============================================================

use hashbrown::HashMap;

use crate::admission::admission_strategy::{AdmissionStrategy};
use crate::utility::structures::CountMinSketch;
use crate::utility::structures::TectonicDoublyLinkedList;
use crate::utility::typings::NodeValue;
use crate::utility::utils::UniqueID;

// ============================================================
// ADMISSION STRATEGY: WINDOW TINY-LFU
// ============================================================
// Admission Policy for admitting entries based on a probabilistic frequency
// calculated using custom Count-Min Sketch structure.
// Candidate are only permitted entry once their estimated frequency exceeds
// configurable minimum frequency.
// ---
// This admission policy is primarily used for specific scenarios:
// * Provides memory efficient, high-quality entry admission
// * Easily adapt to diverse admission patterns
// ---
// Performance Characteristics:
// * "on_get()" -> O(n)
// * "on_insert()" -> O(1)
// * "on_remove()" -> O(1)
// * "should_admit()" -> O(n)
// * Fix size memory footprint
// * Highly cache efficient and scan resistant structure

const DEFAULT_SKETCH_WIDTH: usize = 4096;
const DEFAULT_SKETCH_DEPTH: usize = 4;
const DEFAULT_MIN_FREQUENCY: u8 = 2;
const DEFAULT_WINDOW_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub struct WindowTinyLFUAdmisssion {
    sketch: CountMinSketch,
    window: TectonicDoublyLinkedList,
    window_index: HashMap<UniqueID, NodeValue>,
    window_capacity: usize,
    frequency: u8,
}