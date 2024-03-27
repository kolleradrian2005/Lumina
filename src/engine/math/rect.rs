use super::vec2::Vec2;

pub struct Rect {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
}

impl Rect {
    pub const fn zero() -> Self {
        Rect {
            bottom_left: Vec2::zero(),
            top_right: Vec2::zero(),
        }
    }
}
