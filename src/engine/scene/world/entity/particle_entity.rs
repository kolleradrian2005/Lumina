use std::{
    f32::consts::PI,
    ops::Range,
    time::{Duration, Instant},
};

use rand::{rngs::StdRng, Rng};

use crate::engine::{
    math::vec3::Vec3,
    scene::world::component::{
        model_component::ModelComponent,
    },
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
    pub particle_type: ParticleEntityType,
    pub model: ModelComponent,
    pub spawn_position: Vec3,
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifespan: Option<Duration>,
    pub spawn_time: Instant,
    pub amplitude: f32,
    pub frequency: f32,
    pub offset: f32,
    pub alive: bool,
}

impl ParticleEntity {
    pub fn spawn(
        particle_type: ParticleEntityType,
        mut spawn_position: Vec3,
        model: ModelComponent,
        rng: &mut StdRng,
    ) -> Self {
        let sig = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
        spawn_position.z += sig * 0.01;
        if let ParticleEntityType::Fish = &particle_type {
            const MAX_DIST: f32 = 0.04;
            spawn_position.y += rng.gen_range(-MAX_DIST..MAX_DIST);
        }
        Self {
            spawn_position,
            position: spawn_position,
            velocity: particle_type.default_velocity(),
            lifespan: Duration::from_secs_f32(2.0).into(),
            spawn_time: Instant::now(),
            amplitude: rng.gen_range(particle_type.amplitude_range()),
            frequency: rng.gen_range(particle_type.frequency_range()),
            offset: rng.gen_range(particle_type.offset_range()),
            particle_type,
            alive: true,
            model,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let duration = Instant::now().duration_since(self.spawn_time);
        if let Some(ls) = self.lifespan {
            if ls < duration {
                self.alive = false;
                return;
            }
        }
        match self.particle_type {
            ParticleEntityType::Bubble => {
                self.position += self.velocity * delta_time;
                self.position.x = self.spawn_position.x
                    + ((duration.as_secs_f32() * self.frequency + self.offset).sin()
                        - self.offset.sin())
                        * self.amplitude;
            }
            ParticleEntityType::Fish => {
                self.position += self.velocity * delta_time;
                self.position.y = self.spawn_position.y
                    + ((duration.as_secs_f32() * self.frequency + self.offset).sin())
                        * self.amplitude;
            }
        }
    }

    pub fn set_speed(&mut self, s: f32) {
        self.velocity = self.particle_type.default_velocity() * s;
    }
}
