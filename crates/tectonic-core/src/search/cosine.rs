// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// COSINE DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct Cosine;

impl<const D: usize> SearchMethod<D> for Cosine {
    #[inline(always)]
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
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
}