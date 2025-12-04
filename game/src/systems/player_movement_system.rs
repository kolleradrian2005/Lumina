use lumina_engine::scene::world::{
    component::movement_component::MovementComponent, system::system::System, world::World,
};

use crate::components::player_state_component::PlayerStateComponent;

pub struct PlayerMovementSystem;

impl System for PlayerMovementSystem {
    fn run(&self, world: &mut World, _: f32) {
        for (_, (player_state, movement)) in
            world.query_mut::<(&mut PlayerStateComponent, &mut MovementComponent)>()
        {
            movement.direction = player_state.direction();
            movement.base_acceleration = player_state.acceleration();
        }
    }
}
