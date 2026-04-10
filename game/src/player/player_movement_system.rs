use lumina_engine::logic::{
    ecs::{
        component::force::{AppliedForce, Force, ForceEffect, ForceMode},
        system::system::System,
    },
    scene::world::World,
};

use crate::player::player_state::PlayerState;

pub struct PlayerMovementSystem;

impl System for PlayerMovementSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (player_state, force)) in world.query_mut::<(&mut PlayerState, &mut Force)>() {
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
