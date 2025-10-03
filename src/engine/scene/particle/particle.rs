use std::time::Duration;

use rand::rngs::StdRng;

use crate::engine::{
    math::vec3::Vec3, model::model::Model, texture::resource_provider::ResourceProvider,
};

use super::{bubble::Bubble, fish::Fish};

pub trait Particle {
    //fn spawn(model: Model, spawn_position: Vec3, rng: &mut StdRng) -> Box<dyn Particle>;
    fn update(&mut self, delta_time: f32);
    fn is_alive(&self) -> bool;
    fn get_model(&self) -> &Model;
    fn set_lifespan(&mut self, duration: Option<Duration>);
    fn set_speed(&mut self, s: f32);
}

pub enum ParticleType {
    Bubble,
    Fish,
}

impl ParticleType {
    pub fn spawn(
        &self,
        model: Model,
        spawn_position: Vec3,
        rng: &mut StdRng,
    ) -> Box<dyn Particle + Send + Sync> {
        match self {
            ParticleType::Bubble => Box::new(Bubble::spawn(model, spawn_position, rng)),
            ParticleType::Fish => Box::new(Fish::spawn(model, spawn_position, rng)),
        }
    }
    pub fn create_model(&self, resource_provider: &dyn ResourceProvider) -> Model {
        match self {
            ParticleType::Bubble => resource_provider.get_model("bubble"),
            ParticleType::Fish => resource_provider.get_model("fish"),
        }
    }
}
