use std::sync::mpsc::Receiver;
use glfw::{Context, Window, Glfw, WindowEvent, Action};
use crate::input_handler::InputHandler;
use crate::references;
use crate::scene::Scene;

pub static mut WINDOW_WIDTH: i32 = references::INITIAL_WINDOW_WIDTH;
pub static mut WINDOW_HEIGHT: i32 = references::INITIAL_WINDOW_HEIGHT;

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
            unsafe { WINDOW_WIDTH } as u32,
            unsafe { WINDOW_HEIGHT } as u32,
            references::WINDOW_TITLE,
            glfw::WindowMode::Windowed
        ).expect("Failed to create window.");
        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        let input_handler: InputHandler = InputHandler::init();
        WindowHandler { glfw, window, events, input_handler }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn handle_events(&mut self, scene: &mut Scene, delta_time: u128) {
        let _ = &self.glfw.poll_events();
        let (width, height) = self.window.get_framebuffer_size();
        unsafe {
            if WINDOW_WIDTH != width || WINDOW_HEIGHT != height {
                WINDOW_WIDTH = width;
                WINDOW_HEIGHT = height;
                gl::Viewport(0, 0, width, height);
            }
        };
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
        self.input_handler.handle_keys(scene, delta_time);
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
