// ============================================================
// CUSTOM DTYPES ANNOTATIONS
// ============================================================

pub type DimVector<const D: usize> = [f32; D]; 
pub type SearchVector<const D: usize> = [i8; D];

// ============================================================
// CUSTOM TYPE TRANSFORMATION
// ============================================================

pub fn to_usize(value: u32) -> usize {
    value as usize
}