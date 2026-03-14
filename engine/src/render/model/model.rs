use std::sync::Arc;

use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::render::mesh::Mesh;
use crate::render::resource::texture::{StaticColor, Texture};

#[derive(Clone, Debug)]
pub struct Model {
    // Rendering
    mesh: Arc<Mesh>,
    // Transforming
    position: Vec3,
    rotation: f32,
    scale: Vec2,
    // Texturing
    texture: Texture,
}

impl Model {
    pub fn new(mesh: Mesh) -> Self {
        Model {
            mesh: Arc::new(mesh),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::uniform(1.0),
            texture: StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into(),
        }
    }

    pub fn get_mesh(&self) -> &Arc<Mesh> {
        &self.mesh
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_texture_mut(&mut self) -> &mut Texture {
        &mut self.texture
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
    }

    pub fn get_scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }
}
