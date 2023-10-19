use crate::vec3::Vec3;

pub struct Foreground {
    pub tint_color: Vec3,
    pub tint_intensity: f32,
    pub focal_radius: f32,
    pub darkening_factor: f32,
    pub smooth_factor: f32
}

impl Foreground {
    pub fn construct() -> Self {
        //let mut layer1 = Model::new(vertices, indices, &[]);
        Foreground {
            tint_color: Vec3::new(0.0, 1.0, 1.0),
            tint_intensity: 0.15,
            focal_radius: 0.25,
            darkening_factor: 0.3,
            smooth_factor: 0.2
        }
    }
}