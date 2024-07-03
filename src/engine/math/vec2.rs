use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub const fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub const fn unit() -> Self {
        Vec2 { x: 1.0, y: 1.0 }
    }

    pub const fn uniform(x: f32) -> Self {
        Vec2 { x, y: x }
    }

    pub const fn from(vec: &Vec2) -> Self {
        Vec2 { x: vec.x, y: vec.y }
    }

    pub fn dot(vec1: &Vec2, vec2: &Vec2) -> f32 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }

    pub fn scale(&mut self, scale: &Vec2) {
        self.x *= scale.x;
        self.y *= scale.y;
    }

    pub fn rotate(&mut self, angle: f32) {
        let cos_angle = angle.cos();
        let sin_angle = (-angle).sin();
        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        self.x = new_x;
        self.y = new_y;
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn rotated(&self, angle: f32) -> Vec2 {
        let cos_angle = angle.cos();
        let sin_angle = (-angle).sin();
        let new_x = self.x * cos_angle - self.y * sin_angle;
        let new_y = self.x * sin_angle + self.y * cos_angle;
        Vec2::new(new_x, new_y)
    }

    pub fn normalized(&self) -> Vec2 {
        let length = self.length();
        if length == 0.0 {
            return Vec2::zero();
        }
        Vec2::new(self.x, self.y) / length
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = *self - other;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}
