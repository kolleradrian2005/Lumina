
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}