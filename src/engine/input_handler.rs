use std::collections::HashMap;

use glfw::Key;
use crate::{scene::Scene, vec2::Vec2};

pub struct InputHandler {
    key_states: HashMap<Key, bool>,
    pub exit_requested: bool
}

impl InputHandler {

    pub fn init() -> Self {
        InputHandler { key_states: HashMap::new(), exit_requested: false }
    }

    pub fn update_key_state(&mut self, key_code: Key, state: bool) {
        let entry: &mut bool = self.key_states.entry(key_code).or_insert(state);
        *entry = state;
    }

    pub fn handle_keys(&mut self, scene: &mut Scene, delta_time: u128) {
        //let mut camera_position: Vec2 = scene.camera.get_position().clone();
        let player_position: &Vec2 = scene.player.get_position();
        let mut direction: Vec2 = Vec2::new(0.0, 0.0);
        let move_speed = scene.player.get_move_speed();
        // W pressed
        if *self.key_states.get(&Key::W).unwrap_or(&false) {
            direction.y += 1.0;
        }
        // A pressed
        if *self.key_states.get(&Key::A).unwrap_or(&false) {
            direction.x -= 1.0;
        }
        // S pressed
        if *self.key_states.get(&Key::S).unwrap_or(&false) {
            direction.y -= 1.0;
        }
        // D pressed
        if *self.key_states.get(&Key::D).unwrap_or(&false) {
            direction.x += 1.0;
        }
        // ESC pressed
        if *self.key_states.get(&Key::Escape).unwrap_or(&false) {
            self.exit_requested = true;
        }
        scene.player.change_position(&direction.normalized().times(move_speed * delta_time as f32));
    }
}
