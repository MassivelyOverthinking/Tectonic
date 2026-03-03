// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::result::DimVector;

// ============================================================
// DISTANCE & SEARCH METHODS
// ============================================================


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DistanceMetric {
    Cosine,
    Dot,
    Euclidean,
} 

pub trait SearchMethod<const D: usize>: Send + Sync {
    fn distance(&self, x: &DimVector<D>, y: &DimVector<D>) -> f32;
}

pub trait SearchMethodDyn<const D: usize>: SearchMethod<D> {
    fn clone_box(&self) -> Box<dyn SearchMethodDyn<D>>;
}

impl<const D: usize, T> SearchMethodDyn<D> for T where 
    T: 'static + SearchMethod<D> + Clone,
{
    fn clone_box(&self) -> Box<dyn SearchMethodDyn<D>> {
        Box::new(self.clone())
    }
}

impl<const D: usize> Clone for Box<dyn SearchMethodDyn<D>>{
    fn clone(&self) -> Self {
        self.clone_box()
    }
}