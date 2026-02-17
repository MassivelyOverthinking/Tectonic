

pub fn scalar_quantize<const D: usize>(vec: &[f32], levels: u32) -> [u8; D] {
        let min = vec.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = vec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let scale = (max - min) / (levels as f32 - 1.0);

        let quantized: Vec<u8> = vec.iter()
            .map(|&x| {
                let q = ((x - min) / scale).round();
                q.clamp(0.0, (levels - 1) as f32) as u8
            })
            .collect();
    
        quantized.try_into().expect("Vector length does not match array size D")
    }

    pub fn generate_vector_unique_id(x: u64, y: u64) -> u64 {
        (x << 32) | y
    }