use std::ffi::CString;

use gl::types::GLuint;
use include_assets::NamedArchive;

use super::{shader::Shader, shader_handler, shader_program::ShaderProgram};

pub struct TerrainShader {
    id: GLuint,
    model_location: i32,
    isuphill_location: i32,
    height_location: i32,
}

impl TerrainShader {
    pub unsafe fn new(archive: &NamedArchive) -> Self {
        let fragment_shader = Shader::new(archive, "terrain.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new(archive, "terrain.vert", gl::VERTEX_SHADER);
        let id = shader_handler::load_program(&[vertex_shader, fragment_shader]);
        let shader_program = Self {
            id,
            model_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()),
            ),
            isuphill_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uIsUphill").unwrap()),
            ),
            height_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uHeight").unwrap()),
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

    pub unsafe fn set_isuphill(&self, value: bool) {
        gl::Uniform1i(self.isuphill_location, value as i32);
    }

    pub unsafe fn set_height(&self, height: f32) {
        gl::Uniform1f(self.height_location, height);
    }
}

impl ShaderProgram for TerrainShader {
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
