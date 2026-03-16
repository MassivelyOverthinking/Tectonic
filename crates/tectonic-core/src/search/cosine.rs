// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::utility::typings::{DimVector, SearchVector};
use crate::search::distance::SearchMethod;
// ============================================================
// COSINE DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct Cosine;

impl<const D: usize> SearchMethod<D> for Cosine {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        assert!(x.len() == y.len());
        
        let mut dot_acc = 0.0f32;
        let mut norm_x = 0.0f32;
        let mut norm_y = 0.0f32;

        for (&a, &b) in x.iter().zip(y.iter()) {
            dot_acc += a * b;
            norm_x += a * a;
            norm_y += b * b;
        }

        let denom_sq = norm_x * norm_y;
        if denom_sq <= 0.0 {
            return 0.0;
        }

        dot_acc / denom_sq.sqrt()
    }

    #[inline(always)]
    fn distance_i8(&self, x: &SearchVector<D>, y: &SearchVector<D>) -> i8 {
        assert!(x.len() == y.len());

        let mut dot_acc = 0i8;
        let mut norm_x = 0i8;
        let mut norm_y = 0i8;

        for (&a, &b) in x.iter().zip(y.iter()) {
            dot_acc += a * b;
            norm_x += a * a;
            norm_y += b * b;
        }

        let denom_sq = norm_x * norm_y;
        if denom_sq <= 0 {
            return 0;
        }

        dot_acc / denom_sq.isqrt()
    }
}