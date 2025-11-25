use std::ffi::CString;

use gl::types::GLuint;
use include_assets::NamedArchive;

use crate::math::vec3::Vec3;

use super::{shader::Shader, shader_handler, shader_program::ShaderProgram};

pub struct BackgroundShader {
    id: GLuint,
    model_location: i32,
    color_location: i32,
    color1_location: i32,
    color2_location: i32,
    has_texture_location: i32,
    flipped_location: i32,
}

impl BackgroundShader {
    pub unsafe fn new(archive: &NamedArchive) -> Self {
        let fragment_shader = Shader::new(archive, "background.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new(archive, "background.vert", gl::VERTEX_SHADER);
        let id = shader_handler::load_program(&[vertex_shader, fragment_shader]);
        let shader_program = Self {
            id,
            model_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()),
            ),
            color_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uColor").unwrap()),
            ),
            color1_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uColor1").unwrap()),
            ),
            color2_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uColor2").unwrap()),
            ),
            has_texture_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uHasTexture").unwrap()),
            ),
            flipped_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uFlipped").unwrap()),
            ),
        };
        shader_handler::bind_attributes_to_program(&shader_program, 0, "position");
        shader_handler::bind_attributes_to_program(&shader_program, 1, "uv");
        shader_program
    }

    pub unsafe fn set_model_matrix(&self, matrix: &[[f32; 4]]) {
        gl::UniformMatrix4fv(
            self.model_location,
            1,
            gl::FALSE,
            matrix.as_ptr() as *const f32,
        );
    }

    pub unsafe fn set_flipped(&self, value: bool) {
        gl::Uniform1i(self.flipped_location, value as i32);
    }

    pub unsafe fn set_color(&self, color: Vec3) {
        gl::Uniform3f(self.color_location, color.x, color.y, color.z);
    }

    pub unsafe fn set_color1(&self, color1: Vec3) {
        gl::Uniform3f(self.color1_location, color1.x, color1.y, color1.z);
    }

    pub unsafe fn set_color2(&self, color2: Vec3) {
        gl::Uniform3f(self.color2_location, color2.x, color2.y, color2.z);
    }

    pub unsafe fn set_has_texture(&self, value: bool) {
        gl::Uniform1i(self.has_texture_location, value as i32);
    }
}

impl ShaderProgram for BackgroundShader {
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
