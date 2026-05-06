// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::DimVector;
use crate::search::distance::SearchMethod;
// ============================================================
// DISTANCE METHOD: DOT PRODUCT
// ============================================================
// Dot-product similarity.
// ---
// Higher values indicate greater similarity.
// ---
// Performance:
// - O(D)
// - single linear pass
// - no heap allocation
// - compiler-friendly loop over const generic dimension

#[derive(Debug, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct DotProduct;

impl<const D: usize> SearchMethod<D> for DotProduct {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        debug_assert_eq!(x.len(), y.len());

        let mut acc = 0.0_f32;

        for i in 0..D {
            acc += x[i] * y[i];
        }

        debug_assert!(acc.is_finite(), "Dot product resulted in non-finite value: {}", acc);

        acc
    }

    #[inline(always)]
    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> f32 {
        debug_assert_eq!(x.get_length(), y.get_length());

        let mut acc = 0_u32;

        for (&a, &b) in x.get_iter().zip(y.get_iter()) {
            acc = acc.saturating_add((a as u32) * (b as u32));
        }

        acc as f32
    }

    #[inline(always)]
    fn ordering(&self) -> super::distance::DistanceOrdering {
        super::distance::DistanceOrdering::HigherIsBetter
    }
}
