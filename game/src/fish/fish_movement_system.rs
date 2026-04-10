use lumina_engine::logic::{
    ecs::{component::transform::Transform, system::system::System},
    scene::world::World,
};

use super::fish::Fish;

pub struct FishMovementSystem;

impl System for FishMovementSystem {
    fn run(&mut self, world: &mut World, _delta_time: f32) {
        for (_entity, (transform, _fish)) in world.query_mut::<(&mut Transform, &mut Fish)>() {
            if transform.is_flipped {
                transform.position.x += _delta_time * _fish.speed;
            } else {
                transform.position.x -= _delta_time * _fish.speed;
            }
        }
    }
}
