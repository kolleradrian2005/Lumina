use crate::math::vec3::Vec3;

use super::component::Component;

#[derive(Component, Default)]
pub struct MovementComponent {
    pub direction: Vec3,
    pub base_acceleration: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}
