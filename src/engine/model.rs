use gl::types::*;
use crate::mesh_handler::MeshHandler;

use crate::vec2::Vec2;
use crate::texture::Texture;

pub struct Model {
    vao: GLuint,
    vertex_count: GLsizei,
    position: Vec2,
    rotation: f32,
    scale: f32,
    texture: Option<Texture>
}

impl Model {
    pub fn new(vertices: &[f32], indices: &[u32]) -> Self {
        let vao: GLuint = MeshHandler.create_mesh(vertices, indices);
        Model {
            vao,
            vertex_count: vertices.len() as GLsizei,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
            texture: None
        }
    }
    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub fn get_vertex_count(&self) -> GLsizei {
        self.vertex_count
    }
}
