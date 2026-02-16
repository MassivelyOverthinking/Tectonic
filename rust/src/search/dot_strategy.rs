use crate::search::distance_metric::DistanceMetric;

#[derive(Clone)]
pub struct DotProduct;

impl<const D: usize> DistanceMetric<D> for DotProduct {
    #[inline(always)]
    fn distance(&self, x: &[f32; D], y: &[f32; D]) -> f32 {
        let mut similarity = 0.0;
        for i in 0..D {
            similarity += x[i] * y[i];
        }
        -similarity
    }   
}