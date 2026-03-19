use std::{f32::consts::PI, time::Duration};

use lumina_engine::{
    logic::scene::particle_config::ParticleConfig,
    math::{vec2::Vec2, vec3::Vec3},
};

pub struct Particle;

impl Particle {
    pub fn bubble() -> ParticleConfig {
        ParticleConfig {
            base_velocity: Vec3::new(0.0, 0.03, 0.0),
            oscillation_dir: Vec2::new(1.0, 0.0),
            amplitude_range: 0.0075..0.0125,
            frequency_range: 3.0..5.0,
            offset_range: 0.0..PI * 2.0,
            spawn_jitter: Vec3::new(0.0, 0.0, 0.02),
            lifespan: Some(Duration::new(2, 0)),
            cull_radius: Some(4.0),
        }
    }

    pub fn fish() -> ParticleConfig {
        ParticleConfig {
            base_velocity: Vec3::new(-0.03, 0.0, 0.0),
            oscillation_dir: Vec2::new(0.0, 1.0),
            amplitude_range: 0.0025..0.0050,
            frequency_range: 3.0..5.0,
            offset_range: 0.0..PI * 2.0,
            spawn_jitter: Vec3::new(0.0, 0.04, 0.02),
            lifespan: Some(Duration::new(2, 0)),
            cull_radius: Some(4.0),
        }
    }
}
