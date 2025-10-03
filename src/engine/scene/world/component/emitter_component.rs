use std::time::Duration;

use crate::engine::{
    math::vec3::Vec3,
    scene::{
        world::entity::particle_entity::{ParticleEntity, ParticleEntityType},
    },
};

use super::component::Component;

pub struct TimeOut {
    pub start: f32,
    pub duration: f32,
}

pub struct EmitterComponent {
    pub emitter_type: ParticleEntityType,
    pub particles: Vec<ParticleEntity>,
    pub spawn_position: Vec3,
    pub interval: Duration,
    pub lifespan: Option<Duration>,
    pub particle_lifespan: Option<Duration>,
    pub particle_velocity: f32,
    pub alive: bool,
    pub cycle_time: f32,
    pub now: f32,
    pub timeout: Option<TimeOut>,
}

impl EmitterComponent {
    pub fn create(particle_type: ParticleEntityType, spawn_position: Vec3) -> Self {
        Self {
            particles: Vec::new(),
            spawn_position,
            interval: Duration::from_secs_f32(0.1),
            //interval: Duration::from_secs_f32(0.25),
            lifespan: None,
            particle_lifespan: particle_type.default_lifespan(),
            particle_velocity: 1.0,
            alive: true,
            cycle_time: 0.0,
            now: 0.0,
            timeout: None,
            emitter_type: particle_type,
        }
    }
}

impl From<ParticleEntityType> for EmitterComponent {
    fn from(particle_type: ParticleEntityType) -> Self {
        Self::create(particle_type, Vec3::zero())
    }
}

impl Component for EmitterComponent {}
