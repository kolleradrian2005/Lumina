use std::{ops::Range, time::Duration};

use crate::math::{vec2::Vec2, vec3::Vec3};

#[derive(Clone)]
pub struct ParticleConfig {
    pub base_velocity: Vec3,
    pub oscillation_dir: Vec2,
    pub amplitude_range: Range<f32>,
    pub frequency_range: Range<f32>,
    pub offset_range: Range<f32>,
    pub spawn_jitter: Vec3,
    pub lifespan: Option<Duration>,
    pub cull_radius: Option<f32>,
}
