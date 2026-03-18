// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::DimVector;
use crate::utility::typings::SearchVector;
use crate::search::distance::SearchMethod;

// ============================================================
// DOT PRODUCT DISTANCE METRICS
// ============================================================
#[derive(Clone)]
#[allow(dead_code)]
pub struct DotProduct;

impl<const D: usize> SearchMethod<D> for DotProduct {
    #[inline(always)]
    fn distance_f32(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32 {
        assert!(x.len() == y.len());

        let mut acc = 0.0f32;
        for (&a, &b) in x.iter().zip(y.iter()) {
            acc += a * b;
        }
        acc
    }

    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> u8 {
        assert!(x.vector.len() == y.vector.len());

        let mut acc = 0u8;
        for (&a, &b) in x.vector.iter().zip(y.vector.iter()) {
            acc += a * b;
        }
        acc
    }
}
