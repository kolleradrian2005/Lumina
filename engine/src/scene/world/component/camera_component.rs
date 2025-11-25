use crate::{
    math::{transformation, vec2::Vec2, vec3::Vec3},
    scene::world::entity::entity::Entity,
};

use super::component::Component;

#[derive(Clone)]
pub struct CameraComponent {
    pub position: Vec3,
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub near: f32,
    pub far: f32,
    pub focal_offset: Vec2,
    pub max_distance_from_player: f32,
    pub target_entity: Option<Entity>,
}

impl CameraComponent {
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_view_matrix(self.position)
    }
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        transformation::create_ortographic_projection_matrix(aspect_ratio, self.near, self.far)
    }
}

impl Component for CameraComponent {}
