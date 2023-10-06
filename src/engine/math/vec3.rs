
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn from(vec: &Vec3) -> Self {
        Vec3 { x: vec.x, y: vec.y, z: vec.z }
    }
    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    pub fn add(&mut self, vec: &Vec3) {
        self.x += vec.x;
        self.y += vec.y;
        self.z += vec.z;
    }
    pub fn minus(&self, other: &Vec3) -> Vec3 {
        Vec3::new(&self.x - other.x, &self.y - other.y, &self.z - other.z)
    }
    pub fn plus(&self, other: &Vec3) -> Vec3 {
        Vec3::new(&self.x + other.x, &self.y + other.y, &self.z + other.z)
    }
    pub fn times(&self, scalar: f32) -> Vec3 {
        Vec3::new(&self.x * scalar, &self.y * scalar, &self.z * scalar)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        if length == 0.0 {
            return Vec3::new(0.0, 0.0, 0.0)    
        }
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}