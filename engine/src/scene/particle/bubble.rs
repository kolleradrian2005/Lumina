use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use rand::{rngs::StdRng, Rng};

use crate::{math::vec3::Vec3, model::model::Model, transformable::Transformable};

use super::particle::Particle;

pub struct Bubble {
    model: Model,
    spawn_position: Vec3,
    velocity: Vec3,
    lifespan: Option<Duration>,
    spawn_time: Instant,
    amplitude: f32,
    frequency: f32,
    offset: f32,
    alive: bool,
}

const DEFAULT_VELOCITY: Vec3 = Vec3::new(0.0, 0.03, 0.0);

impl Bubble {
    pub fn spawn(mut model: Model, mut spawn_position: Vec3, rng: &mut StdRng) -> Bubble {
        let sig = rng.gen_range(0..=1) as f32 * 2.0 - 1.0;
        spawn_position.z += sig * 0.01;
        model.set_position(spawn_position);
        Bubble {
            model,
            spawn_position,
            velocity: DEFAULT_VELOCITY,
            lifespan: Duration::from_secs_f32(2.0).into(),
            spawn_time: Instant::now(),
            amplitude: rng.gen_range(0.0075..0.0125),
            frequency: rng.gen_range(3.0..5.0),
            offset: rng.gen_range(0.0..PI * 2.0),
            alive: true,
        }
    }
}

impl Particle for Bubble {
    fn get_model(&self) -> &Model {
        &self.model
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn update(&mut self, delta_time: f32) {
        let duration = Instant::now().duration_since(self.spawn_time);
        if let Some(ls) = self.lifespan {
            if ls < duration {
                self.alive = false;
                return;
            }
        }
        let mut position = self.model.get_position();
        position += self.velocity * delta_time;
        position.x = self.spawn_position.x
            + ((duration.as_secs_f32() * self.frequency + self.offset).sin() - self.offset.sin())
                * self.amplitude;
        self.model.set_position(position);
    }

    fn set_lifespan(&mut self, lifespan: Option<Duration>) {
        self.lifespan = lifespan
    }

    fn set_speed(&mut self, s: f32) {
        self.velocity = DEFAULT_VELOCITY * s;
    }
}
