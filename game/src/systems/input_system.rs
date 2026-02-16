
use lumina_engine::{
    input::input_state::InputState,
    math::vec3::Vec3,
    scene::world::{system::system::System, world::World},
};
use winit::keyboard::{Key, NamedKey};

use crate::components::player_state_component::PlayerStateComponent;

pub struct InputSystem;

impl System for InputSystem {
    fn run(&self, world: &mut World, _: f32) {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        let mut fast_pressed = false;
        if let Some(input_state) = world.get_resource::<InputState>() {
            fast_pressed = input_state.is_pressed(Key::Named(NamedKey::Shift));
            // W pressed
            if input_state.is_pressed(Key::Character("w".into())) {
                direction.y += 1.0;
            }
            // A pressed
            if input_state.is_pressed(Key::Character("a".into())) {
                direction.x -= 1.0;
            }
            // S pressed
            if input_state.is_pressed(Key::Character("s".into())) {
                direction.y -= 1.0;
            }
            // D pressed
            if input_state.is_pressed(Key::Character("d".into())) {
                direction.x += 1.0;
            }
        }
        world
            .query_mut::<(&mut PlayerStateComponent,)>()
            .last()
            .map(|(_, (player_input,))| {
                *player_input = match 0.0 < direction.length() {
                    true => match fast_pressed {
                        true => PlayerStateComponent::FastSwimming { direction },
                        false => PlayerStateComponent::Swimming { direction },
                    },
                    false => PlayerStateComponent::Idle,
                };
            });
    }
}
