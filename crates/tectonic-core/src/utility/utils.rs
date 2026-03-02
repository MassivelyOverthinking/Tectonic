// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D]; 

// ============================================================
// GENERAL UTILITY METHODS & STRUCTS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct VectorID {
    next_id: usize,
}

#[allow(dead_code)]
impl VectorID {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    pub fn get_and_increment(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
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