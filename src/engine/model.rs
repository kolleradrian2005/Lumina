use gl::types::*;
use crate::mesh_handler::MeshHandler;

use crate::texture_handler::TextureHandler;
use crate::vec3::Vec3;
use crate::{transformation, get_time};
use crate::vec2::Vec2;

#[derive(Clone)]
pub struct Model {
    // Rendering
    vao: GLuint,
    vertex_count: GLsizei,
    // Transforming
    position: Vec2,
    rotation: f32,
    scale: f32,
    // Texturig
    color: Vec3,
    texture_name: Option<String>,
    texture_names: Vec<String>,
    animation_time: u128,
    flipped: bool
}

impl Model {
    pub fn new(vertices: &[f32], indices: &[u32], uvs: &[f32]) -> Self {
        let vao: GLuint = MeshHandler.create_mesh(vertices, indices, uvs);
        Model {
            vao,
            vertex_count: indices.len() as GLsizei,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
            color: Vec3::new(0.5, 0.5, 0.5),
            texture_name: None,
            texture_names: vec![],
            animation_time: 0,
            flipped: false
        }
    }
    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub fn get_vertex_count(&self) -> GLsizei {
        self.vertex_count
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_model_matrix(&self.position, &self.rotation, &self.scale)
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }
    
    pub fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
    }

    pub fn get_rotation(&self) -> &f32 {
        &self.rotation
    }

    pub fn load_single_texture(&mut self, texture_handler: &mut TextureHandler, texture_name: &str) {
        let result = texture_handler.load_texture(texture_name);
        if result {
            self.texture_name = Some(texture_name.to_string());
        } else {
            self.texture_name = None;
        }
    }

    pub fn load_animated_texture(&mut self, texture_handler: &mut TextureHandler, texture_names: &[&str], animation_time: u128) {
        for texture_name in texture_names {
            self.animation_time = animation_time;
            let result = texture_handler.load_texture(texture_name);
            if result {
                self.texture_names.push(texture_name.to_string());
            }
        }
    }

    pub fn has_texture(&self) -> bool {
        !self.texture_name.is_none() || 0 < self.texture_names.len()
    }

    pub fn get_current_texture(&mut self) -> Option<&String> {
        if !self.has_texture() {
            return None;
        }
        let texture_count = self.texture_names.len();
        if 0 < texture_count {
            let texture_index: usize = ((get_time() % self.animation_time) / (self.animation_time / texture_count as u128)) as usize;
            let texture_name = self.texture_names.get(texture_index).unwrap(); 
            return Some(texture_name);
        }
        self.texture_name.as_ref()
    }

    pub fn get_color(&self) -> &Vec3 {
        &self.color
    }

    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }
    pub fn set_flipped(&mut self, state: bool) {
        self.flipped = state;
    }
    pub fn is_flipped(&self) -> bool {
        self.flipped
    }
}
