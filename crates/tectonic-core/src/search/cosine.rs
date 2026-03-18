// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::{DimVector};
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
    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> u8 {
        assert!(x.get_length() == y.get_length());

        let mut dot_acc = 0u8;
        let mut norm_x = 0u8;
        let mut norm_y = 0u8;

        for (&a, &b) in x.get_iter().zip(y.get_iter()) {
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