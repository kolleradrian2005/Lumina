use winit::{event::MouseButton, keyboard::Key};

use crate::math::vec2::Vec2;

pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
    MouseEvent { button: MouseButton, pressed: bool },
    MouseMove(Vec2),
    WindowResize { width: i32, height: i32 },
}
