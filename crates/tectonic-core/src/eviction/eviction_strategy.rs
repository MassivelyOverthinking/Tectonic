
#[derive(Debug, Clone)]
pub enum EvictionStrategy {
    Lifo,
    Fifo,
    Lru,
}