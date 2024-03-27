use glfw::{Action, Context, Glfw, GlfwReceiver, Key, MouseButton, PWindow, Window, WindowEvent};

use super::{input_handler::InputHandler, math::vec2::Vec2, references, render::updatable::Updatable};

//pub static mut WINDOW_WIDTH: i32 = references::INITIAL_WINDOW_WIDTH;
//pub static mut WINDOW_HEIGHT: i32 = references::INITIAL_WINDOW_HEIGHT;

pub struct WindowHandler {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    input_handler: InputHandler,
    width: i32,
    height: i32,
    aspect_ratio: f32,
}

impl WindowHandler {
    pub fn new() -> Self {
        // Initialize glfw
        let mut glfw: Glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(6));
        let (width, height) = (references::INITIAL_WINDOW_WIDTH, references::INITIAL_WINDOW_HEIGHT);
        // Create window
        let (mut window, events) = glfw.create_window(
            width as u32,
            height as u32,
            references::WINDOW_TITLE,
            glfw::WindowMode::Windowed
        ).expect("Failed to create window.");
        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        // Create input handler
        let input_handler: InputHandler = InputHandler::init();
        WindowHandler {
            glfw,
            window,
            events,
            input_handler,
            width,
            height,
            aspect_ratio: width as f32 / height as f32
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn handle_events(&mut self, updatables: &mut Vec<Updatable>) {
        self.glfw.poll_events();
        // Check if window is resized
        let (width, height) = self.window.get_framebuffer_size();
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.aspect_ratio = width as f32 / height as f32;
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
            updatables.push(Updatable::Projection);
        }
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::CursorPos(mouse_x, mouse_y) => {
                    self.input_handler.set_mouse_position(Vec2::new(mouse_x as f32, mouse_y as f32));
                }
                glfw::WindowEvent::MouseButton(key, action, _) => {
                    match key {
                        MouseButton::Button1 => self.input_handler.set_l_mouse(action),
                        MouseButton::Button2 => self.input_handler.set_r_mouse(action),
                        _ => {}
                    }
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match action {
                        Action::Repeat => {},
                        Action::Press => self.input_handler.update_key_state(key, true),
                        Action::Release => self.input_handler.update_key_state(key, false),
                    }
                },
                _ => {}
            }
        }
        if self.input_handler.is_pressed(Key::Escape) {
            self.get_window_mut().set_should_close(true);
        }
    }
    
    pub fn get_window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_input_handler(&self) -> &InputHandler {
        &self.input_handler
    }
    
    pub fn get_input_handler_mut(&mut self) -> &mut InputHandler {
        &mut self.input_handler
    }
}
