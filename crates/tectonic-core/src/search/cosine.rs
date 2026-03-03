// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// COSINE DISTANCE METRICS
// ============================================================
#[derive(Clone)]
pub struct Cosine;

impl<const D: usize> SearchMethod<D> for Cosine {
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        
    }
}