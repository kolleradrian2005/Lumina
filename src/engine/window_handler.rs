use std::sync::mpsc::Receiver;
use glfw::{Context, Window, Glfw, WindowEvent, Action};
use crate::input_handler::InputHandler;

const WINDOW_TITLE: &str = "Lumina";

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct WindowHandler {
    glfw: Glfw,
    pub window: Window,
    events: Receiver<(f64, WindowEvent)>,
    input_handler: InputHandler
}

impl WindowHandler {
    
    pub fn new() -> Self {
        let mut glfw: Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw.create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WINDOW_TITLE,
            glfw::WindowMode::Windowed
        ).expect("Failed to create window.");
        window.make_current();
        window.set_key_polling(true);
        let input_handler: InputHandler = InputHandler::init();
        WindowHandler { glfw, window, events, input_handler}
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn handle_events(&mut self) {
        let _ = &self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    if action == Action::Repeat {
                        return; // Ignore to save time
                    }
                    let state: bool = action == Action::Press;
                    self.input_handler.update_key_state(key, state);
                    if self.input_handler.exit_requested {
                        self.window.set_should_close(true);
                    }
                },
                _ => {}
            }
        }
        self.input_handler.handle_keys();
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
