use crate::quantization::strategy::QuantizationStrategyDyn;

#[allow(dead_code)]
enum QuantizationType<const D: usize> {
    Scalar(Box<dyn QuantizationStrategyDyn<D>>),
    Binary(Box<dyn QuantizationStrategyDyn<D>>),
    Product(Box<dyn QuantizationStrategyDyn<D>>),
}