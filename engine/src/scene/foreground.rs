use std::collections::VecDeque;

use noise::{NoiseFn, Perlin};

use crate::{
    math::vec2::Vec2,
    render::{uniformbuffer::PostProcessUniformBuffer},
};

pub struct Foreground {
    pub noise: Perlin,
    pub focal_radius: f32,
    pub focus_speed: f32,
    pub god_rays_noise: VecDeque<f64>,
    pub god_rays_max_count: i32,
    pub loaded_noise_index: i32,
    pub god_rays_min_distance: f32,
}

impl Foreground {
    pub fn construct() -> Self {
        let mut foreground = Foreground {
            noise: Perlin::new(6969),
            focal_radius: 0.25,
            focus_speed: 0.25,
            god_rays_noise: VecDeque::new(),
            god_rays_max_count: 30, // Min 2, first and last are cut out
            loaded_noise_index: 0,
            god_rays_min_distance: 0.2,
        };
        foreground
            .god_rays_noise
            .push_back(foreground.get_noise_value(foreground.loaded_noise_index));
        for i in 1..foreground.god_rays_max_count / 2 + 1 as i32 {
            foreground
                .god_rays_noise
                .push_back(foreground.get_noise_value(foreground.loaded_noise_index + i));
            foreground
                .god_rays_noise
                .push_front(foreground.get_noise_value(foreground.loaded_noise_index - i));
        }
        foreground
    }
    pub fn get_noise_value(&self, x: i32) -> f64 {
        let frequency = 0.2;
        self.noise.get([frequency * x as f64, x as f64 * 0.1])
    }
    pub fn update_god_rays(&mut self, pos: Vec2) {
        let noise_index = (pos.x / self.god_rays_min_distance) as i32;
        let difference = self.loaded_noise_index - noise_index;
        if difference != 0 {
            self.loaded_noise_index = noise_index;
            if 0 < difference {
                let x = self.loaded_noise_index - self.god_rays_max_count / 2;
                self.god_rays_noise.push_front(self.get_noise_value(x));
                self.god_rays_noise.pop_back();
            } else {
                let x = self.loaded_noise_index + self.god_rays_max_count / 2;
                self.god_rays_noise.push_back(self.get_noise_value(x));
                self.god_rays_noise.pop_front();
            }
        }
    }
    pub fn get_light_positions(&self) -> Vec<f32> {
        let mut light_positions = Vec::new();
        for i in 1..self.god_rays_max_count as usize - 1 {
            if self.god_rays_noise[i - 1].abs() < self.god_rays_noise[i].abs()
                && self.god_rays_noise[i].abs() > self.god_rays_noise[i + 1].abs()
            {
                //if 0.6 < self.god_rays_noise[i].abs() {
                let tile_index = self.loaded_noise_index - self.god_rays_max_count / 2 + i as i32;
                light_positions.push(tile_index as f32 * self.god_rays_min_distance);
                //light_positions.push(3.0);
                light_positions.push(0.0);
            }
        }
        light_positions
    }

    pub fn get_default_uniform_buffer(&self) -> PostProcessUniformBuffer {
        PostProcessUniformBuffer {
            tint_color: [0.0, 1.0, 1.0],
            tint_intensity: 0.15,
            darkening_factor: 0.2,
            focal_radius: 0.25,
            smooth_factor: 0.2,
            saturation: 1.25,
            vignette_intensity: 0.15,
        }
    }

    pub fn get_focal_radius(&self) -> f32 {
        self.focal_radius
    }
}
