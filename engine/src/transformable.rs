use super::math::{vec2::Vec2, vec3::Vec3};

pub trait Transformable {
    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, pos: Vec3);
    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, rot: f32);
    fn get_scale(&self) -> Vec2;
    fn set_scale(&mut self, scale: Vec2);
}
