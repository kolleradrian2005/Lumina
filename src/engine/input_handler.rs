use std::collections::HashMap;

use glfw::{Action, Key};

use super::math::vec2::Vec2;

pub struct InputHandler {
    key_states: HashMap<Key, bool>,
    l_mouse: Option<Action>,
    r_mouse: Option<Action>,
    mouse_position: Vec2,
}

impl InputHandler {

    pub fn init() -> Self {
        InputHandler {
            key_states: HashMap::new(),
            l_mouse: None,
            r_mouse: None,
            mouse_position: Vec2::zero(),
        }
    }

    pub fn update_key_state(&mut self, key_code: Key, state: bool) {
        *self.key_states.entry(key_code).or_insert(state) = state;
    }

    pub fn is_pressed(&self, key_code: Key) -> bool {
        *self.key_states.get(&key_code).unwrap_or(&false)
    }

    pub fn handle_l_mouse(&mut self) -> Option<Action> {
        let state = self.l_mouse;
        self.l_mouse = None;
        state
    }

    pub fn handle_r_mouse(&mut self) -> Option<Action> {
        let state = self.r_mouse;
        self.r_mouse = None;
        state
    }

    pub fn set_l_mouse(&mut self, action: Action) {
        self.l_mouse = Some(action);
    }

    pub fn set_r_mouse(&mut self, action: Action) {
        self.r_mouse = Some(action);
    }

    pub fn set_mouse_position(&mut self, mouse_position: Vec2) {
        self.mouse_position = mouse_position;
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn get_normalized_mouse_position(&self, (width, height): (i32, i32)) -> Vec2 {
        (self.mouse_position.x / width as f32 * 2.0, self.mouse_position.y / height as f32 * 2.0).into()
    }
}
