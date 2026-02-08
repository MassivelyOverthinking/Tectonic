
#[repr(C)]
pub struct VectorEntry<const D: usize> {
    /// Unique identifier for the vector entry (Immutable).
    entry_id: String,

    /// High-dimensional vector data (Immutable).
    vector: vec![f32; D],

    /// Optional metadata associated with the vector (Immutable).
    metadata: Option<String>,

    key_hash: u64,
}