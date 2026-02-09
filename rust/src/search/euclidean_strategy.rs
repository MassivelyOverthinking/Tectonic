use crate::search::distance_metric::DistanceMetric;

pub struct EuclideanProduct;

impl<const D: usize> DistanceMetric<D> for EuclideanProduct {
    fn distance(&self, x: &[f32; D], y: &[f32; D]) -> f32 {
        let mut result = 0.0;
        for i in 0..D {
            let distance = x[i] - y[i];
            result += distance * distance;
        }
        result
    }
}