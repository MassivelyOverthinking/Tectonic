use crate::search::distance_metric::DistanceMetric;

pub struct CosineProduct;

impl<const D: usize> DistanceMetric<D> for CosineProduct {
    #[inline(always)]
    fn distance(&self, x: &[f32; D], y: &[f32; D]) -> f32 {
        let mut dot_product = 0.0;
        let mut norm_x = 0.0;
        let mut norm_y = 0.0;

        for i in 0..D {
            dot_product += x[i] * y[i];
            norm_x += x[i] * x[i];
            norm_y += y[i] * y[i];
        }

        if norm_x == 0.0 || norm_y == 0.0 {
            return 1.0; // If either vector is zero, return maximum distance
        }

        1.0 - (dot_product / (norm_x.sqrt() * norm_y.sqrt()))
    }
}