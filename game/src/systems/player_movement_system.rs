use lumina_engine::scene::world::{
    component::force_component::{AppliedForce, ForceComponent, ForceEffect, ForceMode},
    system::system::System,
    world::World,
};

use crate::components::player_state_component::PlayerStateComponent;

pub struct PlayerMovementSystem;

impl System for PlayerMovementSystem {
    fn run(&self, world: &mut World, _: f32) {
        for (_, (player_state, force)) in
            world.query_mut::<(&mut PlayerStateComponent, &mut ForceComponent)>()
        {
            let direction = player_state.direction();
            let magnitude = player_state.acceleration() * force.mass;
            if magnitude > 0.0 {
                force.apply_force(AppliedForce {
                    id: "player_movement".to_string(),
                    effect: ForceEffect::Linear(direction * magnitude),
                    mode: ForceMode::Impulse,
                });
            }
        }
    }
}
