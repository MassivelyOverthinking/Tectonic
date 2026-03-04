// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{result::VectorEntry, storage::location::ArenaLocation};

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
pub struct RepoSlot<'a> {
    pub generation: u32,
    pub vector: Option<ArenaLocation<'a>>
}

impl RepoSlot<'static> {
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