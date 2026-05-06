// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// DISTANCE METHOD: EUCLIDEAN
// ============================================================
// Squared Euclidean distance.
// ---
// Lower values indicate greater similarity.
// ---
// This intentionally returns squared distance instead of true Euclidean
// distance. For nearest-neighbor ranking, `sqrt` is unnecessary because it is
// monotonic and does not change ordering.
 
#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct Euclidean;

impl<const D: usize> SearchMethod<D> for Euclidean {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        debug_assert_eq!(x.len(), y.len());

        let mut acc = 0.0_f32;

        for i in 0..D {
            let delta = x[i] - y[i];
            acc += delta * delta;
        }

        debug_assert!(acc.is_finite(), "Euclidean distance resulted in non-finite value: {}", acc);
        acc.sqrt()
    }

    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> f32 {
        debug_assert_eq!(x.get_length(), y.get_length());

        let mut acc = 0_u32;
        for (&a, &b) in x.get_iter().zip(y.get_iter()) {
            let delta = a as i32 - b as i32;
            acc += acc.saturating_add((delta * delta) as u32)
        }

        acc as f32
    }

    #[inline(always)]
    fn ordering(&self) -> super::distance::DistanceOrdering {
        super::distance::DistanceOrdering::LowerIsBetter
    }
}