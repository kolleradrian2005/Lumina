use crate::math::vec3::Vec3;

use super::component::Component;

#[derive(Component, Default)]
pub struct Movement {
    pub direction: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}
