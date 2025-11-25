use std::ffi::CString;

use gl::types::GLuint;
use include_assets::NamedArchive;

use crate::math::vec3::Vec3;

use super::{shader::Shader, shader_handler, shader_program::ShaderProgram};

pub struct GuiShader {
    id: GLuint,
    model_location: i32,
    aspect_ratio_location: i32,
    has_texture_location: i32,
    color_location: i32,
}

impl GuiShader {
    pub unsafe fn new(archive: &NamedArchive) -> Self {
        let fragment_shader = Shader::new(archive, "gui.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new(archive, "gui.vert", gl::VERTEX_SHADER);
        let shaders = vec![fragment_shader, vertex_shader];
        let id = shader_handler::load_program(&shaders);
        let shader_program = Self {
            id,
            model_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()),
            ),
            aspect_ratio_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uAspectRatio").unwrap()),
            ),
            has_texture_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uHasTexture").unwrap()),
            ),
            color_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uColor").unwrap()),
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
    pub unsafe fn set_aspect_ratio(&self, aspect_ratio: f32) {
        gl::Uniform1f(self.aspect_ratio_location, aspect_ratio);
    }
    pub unsafe fn set_color(&self, color: Vec3) {
        gl::Uniform3f(self.color_location, color.x, color.y, color.z);
    }
    pub unsafe fn set_has_texture(&self, value: bool) {
        gl::Uniform1i(self.has_texture_location, value as i32);
    }
}

impl ShaderProgram for GuiShader {
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
