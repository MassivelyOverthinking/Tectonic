// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::typings::DimVector;
use crate::utility::typings::SearchVector;
use crate::search::distance::SearchMethod;
// ============================================================
// EUCLIDEAN DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct Euclidean;

impl<const D: usize> SearchMethod<D> for Euclidean {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        assert!(x.len() == y.len());

        let mut acc = 0.0f32;
        for (&a, &b) in x.iter().zip(y.iter()) {
            let delta = a - b;
            acc += delta * delta;
        }
        acc.sqrt()
    }

    fn distance_i8(&self, x: &SearchVector<D>, y: &SearchVector<D>) -> i8 {
        assert!(x.len() == y.len());

        let mut acc = 0i8;
        for (&a, &b) in x.iter().zip(y.iter()) {
            let delta = a - b;
            acc += delta * delta;
        }
        acc.isqrt()
    }
}