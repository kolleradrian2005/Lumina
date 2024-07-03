use std::ffi::CString;

use gl::types::GLuint;
use include_assets::NamedArchive;

use crate::engine::{
    math::vec3::Vec3, render::scene_renderer::ObjectType, texture::texture::Texture,
};

use super::{shader::Shader, shader_handler, shader_program::ShaderProgram};

pub struct ModelShader {
    id: GLuint,
    use_tesselation: bool,
    model_location: i32,
    object_type_location: i32,
    //has_texture_location: i32,
    texture_type_location: i32,
    color_location: i32,
    flipped_location: i32,
    terrain_isuphill_location: i32,
    terrain_height_location: i32,
    current_location: i32,
}

impl ModelShader {
    pub unsafe fn new(archive: &NamedArchive, use_tesselation: bool) -> Self {
        let fragment_shader = Shader::new(archive, "model.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new(archive, "model.vert", gl::VERTEX_SHADER);
        let mut shaders = vec![fragment_shader, vertex_shader];
        if use_tesselation {
            let tesc_shader = Shader::new(archive, "model.tesc", gl::TESS_CONTROL_SHADER);
            let tese_shader = Shader::new(archive, "model.tese", gl::TESS_EVALUATION_SHADER);
            shaders.push(tesc_shader);
            shaders.push(tese_shader);
        }
        let id = shader_handler::load_program(&shaders);
        let shader_program = Self {
            id,
            use_tesselation,
            model_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()),
            ),
            object_type_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uObjectType").unwrap()),
            ),
            //has_texture_location: gl::GetUniformLocation(id, std::ffi::CStr::as_ptr(&CString::new("uHasTexture").unwrap())),
            texture_type_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uTextureType").unwrap()),
            ),
            color_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uColor").unwrap()),
            ),
            flipped_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uFlipped").unwrap()),
            ),
            terrain_isuphill_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uTerrainIsUphill").unwrap()),
            ),
            terrain_height_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uTerrainHeight").unwrap()),
            ),
            current_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uCurrent").unwrap()),
            ),
        };
        shader_handler::bind_attributes_to_program(&shader_program, 0, "position");
        shader_handler::bind_attributes_to_program(&shader_program, 1, "uv");
        shader_program
    }
    pub unsafe fn set_model_matrix(&self, matrix: [[f32; 4]; 4]) {
        gl::UniformMatrix4fv(
            self.model_location,
            1,
            gl::FALSE,
            matrix.as_ptr() as *const f32,
        );
    }
    pub unsafe fn set_object_type(&self, object_type: ObjectType) {
        gl::Uniform1i(self.object_type_location, object_type as i32);
    }
    pub unsafe fn set_flipped(&self, value: bool) {
        gl::Uniform1i(self.flipped_location, value as i32);
    }
    pub unsafe fn set_color(&self, color: Vec3) {
        gl::Uniform3f(self.color_location, color.x, color.y, color.z);
    }
    pub unsafe fn set_texture_type(&self, texture: &Texture) {
        let value = match texture {
            Texture::StaticColor(_) => 0,
            Texture::StaticTexture(_) => 1,
            Texture::AnimatedTexture(_) => 1,
            Texture::GradientTexture(_) => 2,
        };
        gl::Uniform1i(self.texture_type_location, value);
    }
    pub unsafe fn set_isuphill(&self, value: bool) {
        gl::Uniform1i(self.terrain_isuphill_location, value as i32);
    }
    pub unsafe fn set_height(&self, height: f32) {
        gl::Uniform1f(self.terrain_height_location, height);
    }
    pub unsafe fn set_current(&self, current: f32) {
        gl::Uniform1f(self.current_location, current);
    }
    pub fn has_tesselation(&self) -> bool {
        self.use_tesselation
    }
}

impl ShaderProgram for ModelShader {
    fn get_id(&self) -> GLuint {
        self.id
    }
    unsafe fn start(&self) {
        shader_handler::start_program(self)
    }
    unsafe fn stop(&self) {
        shader_handler::stop_program()
    }
}
