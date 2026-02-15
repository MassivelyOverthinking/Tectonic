

pub trait DistanceMetric<const D: usize>: Send + Sync {
    /// Compute distance between two vectors of dimension D.
    /// Lower distance indicates higher similarity.
    fn distance(&self, x: &[f32; D], y: &[f32; D]) -> f32;
}