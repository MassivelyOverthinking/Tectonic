// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// EUCLIDEAN DISTANCE METRICS
// ============================================================
#[derive(Clone)]
pub struct Euclidean;

impl<const D: usize> SearchMethod<D> for Euclidean {
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        
    }
}