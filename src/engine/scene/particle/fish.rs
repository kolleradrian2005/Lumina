use std::{f32::consts::PI, time::Duration};

use rand::{rngs::ThreadRng, Rng};

use crate::engine::{math::vec3::Vec3, model::model::Model, transformable::Transformable};

use super::particle::Particle;

pub struct Fish {
    model: Model,
    spawn_position: Vec3,
    velocity: Vec3,
    lifespan: Option<f32>,
    now: f32,
    amplitude: f32,
    frequency: f32,
    offset: f32,
    alive: bool,
}

const DEFAULT_VELOCITY: Vec3 = Vec3::new(-0.03, 0.0, 0.0);

impl Fish {
    pub fn spawn(mut model: Model, mut spawn_position: Vec3, rng: &mut ThreadRng) -> Fish {
        let sig = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
        const MAX_DIST: f32 = 0.04;
        spawn_position.z += sig * 0.01;
        spawn_position.y += rng.gen_range(-MAX_DIST..MAX_DIST);
        model.set_position(spawn_position);
        let dir = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
        if dir == 1.0 {
            model.set_flipped(true);
        }
        Fish {
            model,
            spawn_position,
            velocity: DEFAULT_VELOCITY * dir,
            lifespan: Some(2.0),
            now: 0.0,
            amplitude: rng.gen_range(0.0025..0.0050),
            frequency: rng.gen_range(3.0..5.0),
            offset: rng.gen_range(0.0..PI * 2.0),
            alive: true,
        }
    }
}

impl Particle for Fish {
    fn get_model(&self) -> &Model {
        &self.model
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn update(&mut self, delta_time: f32) {
        self.now += delta_time;
        if let Some(ls) = self.lifespan {
            if ls < self.now {
                self.alive = false;
                return;
            }
        }
        let mut position = self.model.get_position();
        position += self.velocity * delta_time;
        position.y = self.spawn_position.y
            + ((self.now * self.frequency + self.offset).sin()) * self.amplitude;
        self.model.set_position(position);
    }

    fn set_lifespan(&mut self, lifespan: Option<Duration>) {
        self.lifespan = match lifespan {
            Some(ls) => ls.as_secs_f32().into(),
            None => None,
        }
    }

    fn set_speed(&mut self, s: f32) {
        self.velocity = DEFAULT_VELOCITY * s;
    }
}
