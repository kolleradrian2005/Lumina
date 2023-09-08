use std::collections::HashMap;

use glfw::Key;

pub struct InputHandler {
    key_states: HashMap<Key, bool>,
    pub exit_requested: bool
}

impl InputHandler {

    pub fn init() -> Self {
        InputHandler { key_states: HashMap::new(), exit_requested: false }
    }
    
    //pub fn get() -> &'static mut InputHandler {
    //static mut INSTANCE: Option<InputHandler> = None;
    //unsafe {
    //        INSTANCE.get_or_insert_with(|| InputHandler::init())
    //    } 
    //}

    pub fn update_key_state(&mut self, key_code: Key, state: bool) {
        let entry: &mut bool = self.key_states.entry(key_code).or_insert(state);
        *entry = state;
    }

    pub fn handle_keys(&mut self) {
        // W pressed
        if *self.key_states.get(&Key::W).unwrap_or(&false) {
            println!("W is pressed");
        }
        // A pressed
        if *self.key_states.get(&Key::A).unwrap_or(&false) {
            println!("A is pressed");
        }
        // S pressed
        if *self.key_states.get(&Key::S).unwrap_or(&false) {
            println!("S is pressed");
        }
        // D pressed
        if *self.key_states.get(&Key::D).unwrap_or(&false) {
            println!("D is pressed");
        }
        // ESC pressed
        if *self.key_states.get(&Key::Escape).unwrap_or(&false) {
            self.exit_requested = true;
        }


    }
}
