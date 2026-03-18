// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::quantization::quantized_entry::QuantizedEntry;
use crate::utility::typings::DimVector;
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

    fn distance_u8(&self, x: &QuantizedEntry, y: &QuantizedEntry) -> u8 {
        assert!(x.get_length() == y.get_length());

        let mut acc = 0u8;
        for (&a, &b) in x.get_iter().zip(y.get_iter()) {
            let delta = a - b;
            acc += delta * delta;
        }
        acc.isqrt()
    }
}