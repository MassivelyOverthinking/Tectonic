
pub trait QuantizationStrategy<const D: usize>: Send + Sync {
    /// Quantize a vector of dimension D into a byte array.
    fn quantize(&self, vector: &[f32; D]) -> [u8; D];
}

pub trait QuantizationStrategyDyn<const D: usize>: QuantizationStrategy<D> {
    fn clone_box(&self) -> Box<dyn QuantizationStrategyDyn<D>>;
}

impl<const D: usize, T> QuantizationStrategyDyn<D> for T 
where
    T: 'static + QuantizationStrategy<D> + Clone,
{
    fn clone_box(&self) -> Box<dyn QuantizationStrategyDyn<D>> {
        Box::new(self.clone())
    }
}

impl<const D: usize> Clone for Box<dyn QuantizationStrategyDyn<D>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct ScalarQuantization<const D: usize>;

#[derive(Clone)]
pub struct BinaryQuantization<const D: usize>;

#[derive(Clone)]
pub struct ProductQuantization<const D: usize>;

impl<const D: usize> QuantizationStrategy<D> for ScalarQuantization<D> {
    pub fn quantize(&self, vector: &[f32; D]) -> [u8; D] {
        
    };
}

impl<const D: usize> QuantizationStrategy<D> for BinaryQuantization<D> {
    pub fn quantize(&self, vector: &[f32; D]) -> Vec<u8> {
        
    };
}

impl<const D: usize> QuantizationStrategy<D> for ProductQuantization<D> {
    pub fn quantize(&self, vector: &[f32; D]) -> Vec<u8> {
        
    };
}