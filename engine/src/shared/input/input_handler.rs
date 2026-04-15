use winit::event::MouseButton;

use crate::{
    logic::scene::world::World,
    shared::{
        input::{input_event::InputEvent, input_state::InputState},
        window_size::WindowSize,
    },
};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_input_event(world: &mut World, event: InputEvent) {
        match event {
            InputEvent::WindowResize { width, height } => {
                let window_size = world.expect_resource_mut::<WindowSize>();
                window_size.width = width;
                window_size.height = height;
            }
            InputEvent::KeyDown(key) => {
                world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .update_key_state(key, true);
            }
            InputEvent::KeyUp(key) => {
                world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .update_key_state(key, false);
            }
            InputEvent::MouseEvent { button, pressed } => {
                match button {
                    MouseButton::Left => {
                        world
                            .get_resource_mut::<InputState>()
                            .unwrap()
                            .set_l_mouse(pressed);
                    }
                    MouseButton::Right => {
                        world
                            .get_resource_mut::<InputState>()
                            .unwrap()
                            .set_r_mouse(pressed);
                    }
                    _ => {}
                };
            }
            InputEvent::MouseMove(vec2) => world
                .get_resource_mut::<InputState>()
                .unwrap()
                .update_mouse_position(vec2),
        };
    }
}
