

pub trait DistanceMetric<const D: usize>: Send + Sync {
    /// Compute distance between two vectors of dimension D.
    /// Lower distance indicates higher similarity.
    fn distance(&self, x: &[f32; D], y: &[f32; D]) -> f32;
}

pub trait DistanceMetricDyn<const D: usize>: DistanceMetric<D> {
    fn clone_box(&self) -> Box<dyn DistanceMetricDyn<D>>;
}

impl<const D: usize, T> DistanceMetricDyn<D> for T where
    T: 'static + DistanceMetric<D> + Clone,
{
    fn clone_box(&self) -> Box<dyn DistanceMetricDyn<D>> {
        Box::new(self.clone())
    }
}

impl<const D: usize> Clone for Box<dyn DistanceMetricDyn<D>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}