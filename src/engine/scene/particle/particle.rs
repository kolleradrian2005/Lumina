use std::time::Duration;

use rand::rngs::ThreadRng;

use crate::engine::{
    math::vec3::Vec3, model::model::Model, texture::resource_manager::ResourceManager,
};

use super::{bubble::Bubble, fish::Fish};

pub trait Particle {
    //fn spawn(model: Model, spawn_position: Vec3, rng: &mut ThreadRng) -> Box<dyn Particle>;
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
        rng: &mut ThreadRng,
    ) -> Box<dyn Particle> {
        match self {
            ParticleType::Bubble => Box::new(Bubble::spawn(model, spawn_position, rng)),
            ParticleType::Fish => Box::new(Fish::spawn(model, spawn_position, rng)),
        }
    }
    pub fn create_model(&self, resource_manager: &mut ResourceManager) -> Model {
        match self {
            ParticleType::Bubble => resource_manager.get_model("bubble"),
            ParticleType::Fish => resource_manager.get_model("fish"),
        }
    }
}
