// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::typings::DimVector;
use crate::search::distance::SearchMethod;

// ============================================================
// DOT PRODUCT DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct DotProduct;

impl<const D: usize> SearchMethod<D> for DotProduct {
    #[inline(always)]
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        let mut acc = 0.0f32;
        for (&a, &b) in x.iter().zip(y.iter()) {
            acc += a * b;
        }
        acc
    }
}
