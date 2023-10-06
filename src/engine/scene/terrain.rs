use noise::{Perlin, NoiseFn};

extern crate noise;

pub struct Terrain {
    width: usize,
    noise: Perlin
}

impl Terrain {
    pub fn new(width: usize, seed: u32) -> Self {
        let noise = Perlin::new(seed);
        Terrain { width, noise }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self, x: usize) -> f32 {
        if self.width < x {
            return 0.0;
        }
        let normalized_x = x as f64 / self.width as f64;
        let noise_value = self.noise.get([normalized_x, 0.0]);
        let scaled_height = (noise_value * 1.0) as f32;
        scaled_height as f32
    }
}
