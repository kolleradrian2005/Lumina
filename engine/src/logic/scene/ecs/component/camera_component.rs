use crate::{
    logic::scene::ecs::component::component::Component,
    math::{transformation, vec3::Vec3},
};

#[derive(Clone, Component, Debug)]
pub struct CameraComponent {
    pub position: Vec3,
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraComponent {
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_view_matrix(self.position)
    }
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> [[f32; 4]; 4] {
        transformation::create_ortographic_projection_matrix(aspect_ratio, self.near, self.far)
    }
}
