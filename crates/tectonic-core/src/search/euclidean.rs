// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// EUCLIDEAN DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct Euclidean;

impl<const D: usize> SearchMethod<D> for Euclidean {
    #[inline(always)]
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        let mut acc = 0.0f32;
        for (&a, &b) in x.iter().zip(y.iter()) {
            let delta = a - b;
            acc += delta * delta;
        }
        acc.sqrt()
    }
}