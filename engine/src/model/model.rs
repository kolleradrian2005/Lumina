use std::sync::Arc;

use crate::collider::Collider;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::texture::texture::{StaticColor, Texture};
use crate::transformable::Transformable;

use super::mesh::Mesh;

#[derive(Clone, Debug)]
pub struct Model {
    // Rendering
    mesh: Arc<Mesh>,
    flipped: bool,
    // Transforming
    position: Vec3,
    rotation: f32,
    scale: Vec2,
    // Texturing
    texture: Texture,
    collider: Option<Collider>,
}

impl Model {
    pub fn new(vertices: &[f32], indices: &[u32], uvs: &[f32]) -> Self {
        Model {
            mesh: Arc::new(Mesh::new(vertices, indices, uvs)),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::uniform(1.0),
            texture: StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into(),
            flipped: false,
            collider: None,
        }
    }

    pub fn get_collider(&self) -> &Option<Collider> {
        &self.collider
    }

    pub fn get_mesh(&self) -> &Arc<Mesh> {
        &self.mesh
    }

    pub fn set_flipped(&mut self, state: bool) {
        if self.flipped != state {
            self.flipped = state;
            if let Some(collider) = &mut self.collider {
                collider.set_flipped(state);
            }
        }
    }

    pub fn is_flipped(&self) -> bool {
        self.flipped
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
}

impl Transformable for Model {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        if let Some(collider) = &mut self.collider {
            collider.set_position(pos.xy());
        }
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
        if let Some(collider) = &mut self.collider {
            collider.set_rotation(rot);
        }
    }

    fn get_scale(&self) -> Vec2 {
        self.scale
    }

    fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        if let Some(collider) = &mut self.collider {
            collider.set_scale(scale);
        }
    }
}
