use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use super::input_handler::InputHandler;

pub struct WindowHandler {
    window: Window,
    input_handler: InputHandler,
    should_close: bool,
}

impl WindowHandler {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop_opt = EventLoop::new();
        if event_loop_opt.is_err() {
            panic!("{:?}", event_loop_opt.unwrap_err());
        }
        let event_loop = event_loop_opt.unwrap();
        let window_opt = WindowBuilder::new().build(&event_loop);
        if window_opt.is_err() {
            panic!("{:?}", window_opt.unwrap_err());
        }
        let window = window_opt.unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        // Create input handler
        let input_handler: InputHandler = InputHandler::init();
        (
            WindowHandler {
                window,
                input_handler,
                should_close: false,
            },
            event_loop,
        )
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn get_window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
    /*
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
    */

    pub fn get_input_handler(&self) -> &InputHandler {
        &self.input_handler
    }
    pub fn get_input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }
}
