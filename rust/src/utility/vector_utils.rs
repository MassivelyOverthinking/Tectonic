use std::cmp::Ordering;

pub fn compare_vectors<const D: usize>(x: &[f32; D], y: &[f32; D]) -> Ordering {
    assert_eq!(x.len(), y.len());

    let mut sum_x = 0.0f32;
    let mut sum_y = 0.0f32;

    for num in 0..D {
        sum_x += x[num];
        sum_y += y[num];
    }

    return sum_x.total_cmp(&sum_y);
}