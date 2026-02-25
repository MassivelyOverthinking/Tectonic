// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::storage::partition::CachePartition;

// ============================================================
// INTERNAL STORE (PARTITIONS + SHARDS)
// ============================================================

pub struct CacheRepo {
    pub vector_repo: Vec<CachePartition>,
}

