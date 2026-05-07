// ============================================================
// IMPORTS AND MODULES
// ============================================================

use crate::{error::TectonicError, quantization::quantized_entry::QuantizedEntry};
use crate::utility::typings::TectonicResult;

use std::arch::x86_64::*;

// ============================================================
// SCALAR QUANTIZATION
// ============================================================


pub fn quantize(input: &[f32]) -> TectonicResult<QuantizedEntry> {
    if input.is_empty() {
        return Err(TectonicError::quantization("Quantizationn input is empty!"));
    }

    for &value in input.iter() {
        if !value.is_finite() {
            return Err(TectonicError::quantization("One values is non-finite in quantization"));
        }
    }

    let (min, max) = find_min_max(input);

    if min == max {
        return Ok(QuantizedEntry {
            vector: vec![0; input.len()],
        });
    }

    let scale = 255.0 / (max - min);

    let values = if is_x86_feature_detected!("avx2") {
        unsafe { quantize_avx2(input, min, max, scale) }
    } else {
        quantize_scalar(input, min, max, scale)
    };

    Ok(QuantizedEntry {
        vector: values,
    })
}

fn find_min_max(input: &[f32]) -> (f32, f32) {
    let mut min = input[0];
    let mut max = input[0];

    for &x in &input[1..] {
        min = min.min(x);
        max = max.max(x);
    }

    (min, max)
}

fn quantize_scalar(input: &[f32], min: f32, max: f32, scale: f32) -> Vec<u8> {
    input
        .iter()
        .map(|&x| {
            let x = x.clamp(min, max);
            let q = ((x - min) * scale).round().clamp(0.0, 255.0);
            q as u8
        })
        .collect()
}

#[target_feature(enable = "avx2")]
unsafe fn quantize_avx2(input: &[f32], min: f32, max: f32, scale: f32) -> Vec<u8> {
    let mut output = vec![0u8; input.len()];

    let min_v = _mm256_set1_ps(min);
    let max_v = _mm256_set1_ps(max);
    let scale_v = _mm256_set1_ps(scale);
    let zero = _mm256_set1_ps(0.0);
    let max255 = _mm256_set1_ps(255.0);

    let mut i = 0;
    let len = input.len();

    while i + 8 <= len {
        let x = _mm256_loadu_ps(input.as_ptr().add(i));

        // clamp
        let x = _mm256_min_ps(_mm256_max_ps(x, min_v), max_v);

        // normalize
        let q = _mm256_mul_ps(_mm256_sub_ps(x, min_v), scale_v);

        // round
        let q = _mm256_round_ps(q, _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC);

        // clamp again
        let q = _mm256_min_ps(_mm256_max_ps(q, zero), max255);

        // convert to i32
        let ints = _mm256_cvtps_epi32(q);

        // extract + pack manually (fast enough for most cases)
        let mut tmp = [0i32; 8];
        _mm256_storeu_si256(tmp.as_mut_ptr() as *mut __m256i, ints);

        for j in 0..8 {
            output[i + j] = tmp[j] as u8;
        }

        i += 8;
    }

    // tail
    for j in i..len {
        let x = input[j].clamp(min, max);
        let q = ((x - min) * scale).round().clamp(0.0, 255.0);
        output[j] = q as u8;
    }

    output
}