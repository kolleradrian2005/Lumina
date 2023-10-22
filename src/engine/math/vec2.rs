
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    pub fn from(vec: &Vec2) -> Self {
        Vec2 { x: vec.x, y: vec.y }
    }
    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
    pub fn add(&mut self, vec: &Vec2) {
        self.x += vec.x;
        self.y += vec.y;
    }
    pub fn minus(&self, other: &Vec2) -> Vec2 {
        Vec2::new(&self.x - other.x, &self.y - other.y)
    }
    pub fn plus(&self, other: &Vec2) -> Vec2 {
        Vec2::new(&self.x + other.x, &self.y + other.y)
    }
    pub fn times(&self, scalar: f32) -> Vec2 {
        Vec2::new(&self.x * scalar, &self.y * scalar)
    }
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn rotated(&self, angle: f32) -> Vec2 {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        Vec2::new(new_x, new_y)
    }
    pub fn normalized(&self) -> Vec2 {
        let length = self.length();
        if length == 0.0 {
            return Vec2::new(0.0, 0.0)    
        }
        Vec2::new(self.x / length, self.y / length)
    }
}