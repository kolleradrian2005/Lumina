use std::time::Duration;

use crate::{
    logic::{
        ecs::{component::component::Component, entity::particle_entity::ParticleEntity},
        scene::particle_config::ParticleConfig,
    },
    math::vec3::Vec3,
};

pub struct TimeOut {
    pub start: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct Emitter {
    pub particle_config: ParticleConfig,
    pub particles: Vec<ParticleEntity>,
    pub spawn_position: Vec3,
    pub interval: Duration,
    pub lifespan: Option<Duration>,
    pub alive: bool,
    pub cycle_time: f32,
    pub now: f32,
    pub timeout: Option<TimeOut>,
    pub cull_radius: Option<f32>,
}

impl Emitter {
    pub fn create(particle_config: ParticleConfig, spawn_position: Vec3) -> Self {
        Self {
            particles: Vec::new(),
            spawn_position,
            interval: Duration::from_secs_f32(0.25),
            lifespan: None,
            alive: true,
            cycle_time: 0.0,
            now: 0.0,
            timeout: None,
            particle_config,
            cull_radius: Some(4.0),
        }
    }
}

impl From<ParticleConfig> for Emitter {
    fn from(particle_config: ParticleConfig) -> Self {
        Self::create(particle_config, Vec3::zero())
    }
}
