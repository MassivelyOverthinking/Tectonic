// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;
use crate::search::distance::SearchMethod;

// ============================================================
// DOT PRODUCT DISTANCE METRICS
// ============================================================
#[derive(Clone)]
pub struct DotProduct;

impl<const D: usize> SearchMethod<D> for DotProduct {
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        
    }
}
