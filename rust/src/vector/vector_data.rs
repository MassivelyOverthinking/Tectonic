
#[repr(align(32))]
pub struct VectorData {
    /// Unique identifier for the vector data (Immutable).
    pub data_id: u64,

    /// Number of times the vector data has been accessed (Mutable).
    pub access_count: u64,

    /// Timestamp for last time vector entry was accessed (Mutable).
    pub last_accessed: u64, 

    /// Total latency accumulated from all accesses (Mutable).
    pub combined_latency: u64,

    /// Average latency per access (Mutable).
    pub average_latency: f64,
}

impl VectorData {
    pub fn new(id: u64) -> Self {
        Self {
            data_id: id,
            access_count: 0,
            last_accessed: 0,
            combined_latency: 0,
            average_latency: 0.0,
        }
    }
}