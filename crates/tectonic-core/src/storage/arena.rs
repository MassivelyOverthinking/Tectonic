use std::collections::VecDeque;


#[derive(Debug, Clone)]
pub struct VectorArena<const D: usize> {
    next_index: usize,
    free_list: VecDeque<usize>,
    capacity: usize,
    size: usize,
    arena: [f32; D]
}

impl<const D: usize> VectorArena<D> {

    fn insert(&mut self, value: f32) -> bool {
        todo!()
    }

    fn generate_id(&self) -> usize {
        todo!()
    }

    pub fn load_factor(&self) -> f32 {
        (self.size as f32 / self.capacity as f32) * 100.0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.size > self.capacity
    }
}