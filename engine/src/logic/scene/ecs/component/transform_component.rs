use crate::math::{vec2::Vec2, vec3::Vec3};

use super::component::Component;

#[derive(Component, Default, Clone, Debug)]
pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
    pub is_flipped: bool,
}
