
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Eviction {
    LIFO,
    FIFO,
    LRU,
    MRU,
    ARC,
    LIRS
}

#[allow(dead_code)]
pub trait EvictionStrategy {
    fn on_get(&mut self);

    fn on_remove(&mut self);

    fn on_insert(&mut self);

    fn get_victim(&mut self);

    fn evict_victim(&mut self);
}