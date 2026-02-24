
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