// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CACHE LOCATION
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Location {
    entry_id: usize,
    entry_index: usize,
}

#[allow(dead_code)]
impl Location {
    pub fn new(id: usize, index: usize) -> Self {
        Self { 
            entry_id: id, 
            entry_index: index 
        }
    }

    pub fn get_id(&self) -> &usize {
        &self.entry_id
    }

    pub fn get_index(&self) -> &usize {
        &self.entry_index
    }
}