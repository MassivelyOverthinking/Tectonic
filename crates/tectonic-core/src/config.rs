#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub num_partitions: usize,
    pub num_shards: usize,
    pub quantization_enabled: bool,
    pub search: todo!(),
    pub eviction: todo!(),
    pub routing: todo!(),
    pub maintenance: todo!(),
    pub metrics: todo!(),
}