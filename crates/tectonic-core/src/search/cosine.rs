// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::{DimVector};
use crate::search::distance::SearchMethod;

// ============================================================
// DISTANCE METHOD: COSINE
// ============================================================
// Cosine similarity.
//
// Higher values indicate greater similarity.
//
// Return range is approximately:
// - `1.0` for same direction
// - `0.0` for orthogonal vectors
// - `-1.0` for opposite direction
//
// Zero vectors return `0.0`.

#[derive(Clone, Copy, Debug, Default)]
#[allow(dead_code)]
pub struct Cosine;

impl<const D: usize> SearchMethod<D> for Cosine {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        debug_assert_eq!(x.len(), y.len());
        
        let mut dot = 0.0_f32;
        let mut norm_x = 0.0_f32;
        let mut norm_y = 0.0_f32;

        for i in 0..D {
            let x_value = x[i];
            let y_value = y[i];

            dot += x_value * y_value;
            norm_x += x_value * x_value;
            norm_y += y_value * y_value;
        }

        let denom_sq = norm_x * norm_y;

        if denom_sq <= f32::EPSILON {
            return 0.0;
        }

        let result = dot / denom_sq.sqrt();

        debug_assert!(result.is_finite(), "Cosine distance resulted in non-finite value: {}", result);

        result.clamp(-1.0, 1.0)
    }

    #[inline(always)]
    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> f32 {
        debug_assert_eq!(x.get_length(), y.get_length());

        let mut dot = 0_u64;
        let mut norm_x = 0_u64;
        let mut norm_y = 0_u64;

        for (&a, &b) in x.get_iter().zip(y.get_iter()) {
            let a = a as u64;
            let b = b as u64;

            dot = dot.saturating_add(a * b);
            norm_x = norm_x.saturating_add(a * a);
            norm_y = norm_y.saturating_add(b * b);
        }

        let denom_sq = norm_x.saturating_mul(norm_y);

         if denom_sq == 0 {
            return 0.0;
        }

        let result = dot as f32 / (denom_sq as f32).sqrt();

        debug_assert!(result.is_finite(), "Cosine distance resulted in non-finite value: {}", result);

        result.clamp(0.0, 1.0)
    }

    fn ordering(&self) -> super::distance::DistanceOrdering {
        super::distance::DistanceOrdering::HigherIsBetter
    }
}