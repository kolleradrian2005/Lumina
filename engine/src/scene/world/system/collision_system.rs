use crate::scene::world::{
    component::{collider_component::ColliderComponent, transform_component::TransformComponent},
    world::World,
};

use super::system::System;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn run(&self, world: &mut World, _: f32) {}
}
