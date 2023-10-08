use noise::{NoiseFn, Perlin};

extern crate noise;

pub struct Terrain {
    width: usize, // Must be odd
    noise: Perlin,
}

impl Terrain {
    pub fn new(width: usize, seed: u32) -> Self {
        let noise = Perlin::new(seed);
        // TODO: width must be odd
        Terrain { width, noise }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self, x: i32) -> f32 {
        let mapped_x = x + ((self.width as i32) + 1) / 2;
        if (self.width as i32) < mapped_x {
            return 0.0;
        }
        let normalized_x = mapped_x as f64 / self.width as f64;
        let noise_value = self.noise.get([normalized_x, 0.0]);
        let scaled_height = (noise_value * 1.0) as f32;
        scaled_height as f32
    }
}
