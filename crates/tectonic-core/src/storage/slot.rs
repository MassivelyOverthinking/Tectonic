// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{result::VectorEntry, storage::location::RepoLocation};

// ============================================================
// INTERNAL SLOTS
// ============================================================

#[derive(Debug, Clone, Copy)]
pub struct ArenaSlot<const D: usize> {
    pub generation: u32,
    pub vector: Option<VectorEntry<D>>
}

impl<const D: usize> ArenaSlot<D> {
    pub fn default() -> Self {
        Self { 
            generation: 1,
            vector: None
        }
    }

    pub fn get_and_increment(&mut self) -> u32 {
        let gen_id = self.generation;
        self.generation += 1;
        gen_id
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct RepoSlot {
    pub generation: u32,
    pub location: Option<RepoLocation>
}

#[allow(dead_code)]
impl RepoSlot {
    pub fn default() -> Self {
        Self { 
            generation: 1,
            location: None
        }
    }

    pub fn get_and_increment(&mut self) -> u32 {
        let gen_id = self.generation;
        self.generation += 1;
        gen_id
    }
}