use crate::scene::world::{
    component::{
        movement_component::MovementComponent, player_state_component::PlayerStateComponent,
    },
    world::World,
};

use super::system::System;

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
