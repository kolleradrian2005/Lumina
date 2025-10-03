use std::time::Duration;

use rand::rngs::StdRng;

use crate::engine::{
    math::vec3::Vec3, model::model::Model, scene::world::terrain::Terrain,
    texture::resource_manager::ResourceManager, transformable::Transformable,
};

use super::particle::{Particle, ParticleType};

struct TimeOut {
    pub start: f32,
    pub duration: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Box<dyn Particle + Send + Sync>>,
    particle_model: Model,
    particle_type: ParticleType, // TODO: use baseparticle as reference and clone it
    spawn_position: Vec3,
    interval: Duration,
    lifespan: Option<Duration>,
    particle_lifespan: Option<Duration>,
    particle_velocity: f32,
    alive: bool,
    cycle_time: f32,
    now: f32,
    timeout: Option<TimeOut>,
}

impl ParticleSystem {
    pub fn spawn(
        particle_type: ParticleType,
        spawn_position: Vec3,
        resource_provider: &mut dyn ResourceProvider,
    ) -> Self {
        ParticleSystem {
            particles: Vec::new(),
            particle_model: particle_type.create_model(resource_provider),
            particle_type,
            spawn_position,
            interval: Duration::from_secs_f32(0.25),
            lifespan: None,
            particle_lifespan: None,
            particle_velocity: 1.0,
            alive: true,
            cycle_time: 0.0,
            now: 0.0,
            timeout: None,
        }
    }

    pub fn set_lifespan(&mut self, lifespan: Option<Duration>) {
        self.lifespan = lifespan;
    }

    pub fn set_particle_lifespan(&mut self, particle_lifespan: Option<Duration>) {
        self.particle_lifespan = particle_lifespan;
    }

    pub fn set_particle_velocity(&mut self, s: f32) {
        self.particle_velocity = s;
    }

    pub fn update(&mut self, delta_time: f32, rng: &mut StdRng, terrain: &Terrain) {
        self.cycle_time += delta_time;
        self.now += delta_time;
        let mut should_spawn = true;
        if let Some(sp) = self.lifespan {
            should_spawn = self.now <= sp.as_secs_f32();
        }
        let mut has_loaded = should_spawn;
        self.particles.retain_mut(|particle| {
            particle.update(delta_time);
            if !has_loaded && terrain.is_loaded(particle.get_model().get_position()) {
                has_loaded = true;
            }
            particle.is_alive()
        });
        if should_spawn {
            let count = self.cycle_time / self.interval.as_secs_f32();
            for i in 0..count as usize {
                let mut particle =
                    self.particle_type
                        .spawn(self.particle_model.clone(), self.spawn_position, rng);
                particle.set_lifespan(self.particle_lifespan);

                particle.set_speed(self.particle_velocity);
                particle.update(i as f32 * self.interval.as_secs_f32());
                self.particles.push(particle);
            }
            self.cycle_time -= count.floor() * self.interval.as_secs_f32();
        }
        if let Some(timeout) = &self.timeout {
            if self.now - timeout.start > timeout.duration {
                self.particles.clear()
            }
        }
        if !has_loaded {
            self.particles.clear();
        }
        if !should_spawn && self.particles.is_empty() {
            self.alive = false;
        }
    }

    pub fn get_spawn_position(&self) -> Vec3 {
        self.spawn_position
    }

    pub fn set_spawn_position(&mut self, spawn_position: Vec3) {
        self.spawn_position = spawn_position
    }

    pub fn is_alive(&mut self) -> bool {
        self.alive
    }

    pub fn set_model_flipped(&mut self, state: bool) {
        self.particle_model.set_flipped(state);
    }

    pub fn set_timeout(&mut self, duration: Option<Duration>) {
        match duration {
            Some(duration) => {
                self.timeout = TimeOut {
                    start: self.now,
                    duration: duration.as_secs_f32(),
                }
                .into()
            }
            None => self.timeout = None,
        };
    }
}
