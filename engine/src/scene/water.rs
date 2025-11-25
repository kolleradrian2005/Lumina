use std::time::Instant;

use noise::{NoiseFn, Perlin};

use crate::math::vec3::Vec3;

pub struct Water {
    resistance: f32,
    current_noise: Perlin,
    current_start: Instant,
}

impl Water {
    pub fn create(seed: u32) -> Self {
        Water {
            resistance: 0.9,
            current_noise: Perlin::new(seed),
            current_start: Instant::now(),
        }
    }
    pub fn get_resistance(&self) -> f32 {
        self.resistance
    }
    pub fn get_current(&self, pos: &Vec3) -> f32 {
        let speed = 0.5;
        let time = Instant::now()
            .duration_since(self.current_start)
            .as_secs_f32();
        self.current_noise
            .get([pos.x as f64, pos.y as f64, speed * time as f64]) as f32
    }
}
