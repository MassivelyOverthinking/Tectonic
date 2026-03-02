// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CUSTOM ERROS (TECTONIC-ERROR)
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

pub fn calculate_sizes(max_entries: usize, partitions: usize) -> Vec<usize> {
    let base_value = max_entries / partitions;
    let remainder_value = max_entries % partitions;

    let mut sizes = vec![base_value; partitions];

    for size in &mut sizes[..remainder_value] {
        *size += 1;
    }

    sizes
}