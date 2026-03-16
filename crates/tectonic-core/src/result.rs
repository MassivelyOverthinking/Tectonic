// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{metrics::vector_metric::VectorMetrics, utility::utils::UniqueID};
use crate::utility::typings::DimVector;

// ============================================================
// CUSTOM RESULT STRUCTURES
// ============================================================

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VectorEntry<const D: usize> {
    pub vector_id: UniqueID,
    pub user_id: Option<String>,
    pub vector: DimVector<D>,
    pub metrics: Option<VectorMetrics<D>>
}

impl<const D: usize> VectorEntry<D> {
    pub fn new(id: usize, generation: u32, user_id: Option<String>, vector: DimVector<D>, metrics_enabled: bool) -> Self {
        Self { 
            vector_id: UniqueID::new(id, generation),
            user_id: user_id,
            vector,
            metrics: if metrics_enabled { Some(VectorMetrics::default()) } else { None },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VectorResult<const D: usize> {
   pub size: usize,
   pub vectors: Vec<DimVector<D>>,
}

impl<const D: usize> VectorResult<D> {
    pub fn new(num_vectors: usize, result_vectors: Vec<DimVector<D>>) -> Self {
        Self { 
            size: num_vectors,
            vectors: result_vectors, 
        }
    }
}