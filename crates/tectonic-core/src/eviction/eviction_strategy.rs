
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EvictionStrategy {
    Lifo,
    Fifo,
    Lru,
}