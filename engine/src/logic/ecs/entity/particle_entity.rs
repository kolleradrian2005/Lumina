use std::{
    f32::consts::PI,
    ops::Range,
    time::{Duration, Instant},
};

use rand::{rngs::StdRng, Rng};

use crate::{
    logic::{ecs::component::model::Model, scene::particle_config::ParticleConfig},
    math::vec3::Vec3,
};

#[derive(Clone)]
pub enum ParticleEntityType {
    Bubble,
    Fish,
}

const BUBBLE_RANGE: Range<f32> = 0.0..PI * 2.0;
const FISH_RANGE: Range<f32> = 0.0..PI * 2.0;

impl ParticleEntityType {
    pub const fn default_velocity(&self) -> Vec3 {
        match &self {
            ParticleEntityType::Bubble => Vec3::new(0.0, 0.03, 0.0),
            ParticleEntityType::Fish => Vec3::new(-0.03, 0.0, 0.0),
        }
    }

    pub const fn amplitude_range(&self) -> Range<f32> {
        match &self {
            ParticleEntityType::Bubble => 0.0075..0.0125,
            ParticleEntityType::Fish => 0.0025..0.0050,
        }
    }

    pub const fn frequency_range(&self) -> Range<f32> {
        match &self {
            ParticleEntityType::Bubble => 3.0..5.0,
            ParticleEntityType::Fish => 3.0..5.0,
        }
    }
    pub const fn offset_range(&self) -> Range<f32> {
        match &self {
            ParticleEntityType::Bubble => BUBBLE_RANGE,
            ParticleEntityType::Fish => FISH_RANGE,
        }
    }

    pub const fn default_lifespan(&self) -> Option<Duration> {
        match &self {
            ParticleEntityType::Bubble => Some(Duration::new(2, 0)),
            ParticleEntityType::Fish => Some(Duration::new(2, 0)),
        }
    }
}

pub struct ParticleEntity {
    pub config: ParticleConfig,
    pub model: Model,
    pub spawn_position: Vec3,
    pub position: Vec3,
    pub velocity: Vec3,
    pub spawn_time: Instant,
    pub amplitude: f32,
    pub frequency: f32,
    pub offset: f32,
    pub alive: bool,
}

impl ParticleEntity {
    pub fn spawn(
        config: ParticleConfig,
        mut spawn_position: Vec3,
        model: Model,
        rng: &mut StdRng,
    ) -> Self {
        if config.spawn_jitter.x > 0.0 {
            spawn_position.x += rng.gen_range(-config.spawn_jitter.x..config.spawn_jitter.x);
        }
        if config.spawn_jitter.y > 0.0 {
            spawn_position.y += rng.gen_range(-config.spawn_jitter.y..config.spawn_jitter.y);
        }
        if config.spawn_jitter.z > 0.0 {
            spawn_position.z += rng.gen_range(-config.spawn_jitter.z..config.spawn_jitter.z);
        }
        Self {
            spawn_position,
            position: spawn_position,
            velocity: config.base_velocity,
            spawn_time: Instant::now(),
            amplitude: rng.gen_range(config.amplitude_range.clone()),
            frequency: rng.gen_range(config.frequency_range.clone()),
            offset: rng.gen_range(config.offset_range.clone()),
            config,
            alive: true,
            model,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let duration = Instant::now().duration_since(self.spawn_time);
        if let Some(ls) = self.config.lifespan {
            if ls < duration {
                self.alive = false;
                return;
            }
        }

        let t = duration.as_secs_f32();

        let wave = (t * self.frequency + self.offset).sin();
        let displacement = wave * self.amplitude;
        let dir = self.config.oscillation_dir;

        self.position = self.spawn_position
            + self.config.base_velocity * t
            + Vec3::new(dir.x * displacement, dir.y * delta_time, 0.0);
    }
}
