use std::ffi::CString;

use gl::types::GLuint;
use include_assets::NamedArchive;

use crate::{
    math::vec2::Vec2,
    shader::{parameter_schema::ParameterSchema, shader_parameter_type::ShaderParameterType},
};

use super::{shader::Shader, shader_handler, shader_program::ShaderProgram};

pub struct PostprocessShader {
    id: GLuint,
    focal_offset_location: i32,
    aspect_ratio_location: i32,
    num_lights_location: i32,
    light_positions_location: i32,
}

impl PostprocessShader {
    pub unsafe fn new(archive: &NamedArchive) -> Self {
        let fragment_shader = Shader::new(archive, "postprocess.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new(archive, "postprocess.vert", gl::VERTEX_SHADER);
        let id = shader_handler::load_program(&[vertex_shader, fragment_shader]);
        let shader_program = Self {
            id,
            focal_offset_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uFocalOffset").unwrap()),
            ),
            aspect_ratio_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uAspectRatio").unwrap()),
            ),
            num_lights_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uNumLights").unwrap()),
            ),
            light_positions_location: gl::GetUniformLocation(
                id,
                std::ffi::CStr::as_ptr(&CString::new("uLightPositions").unwrap()),
            ),
        };
        shader_handler::bind_attributes_to_program(&shader_program, 0, "position");
        shader_handler::bind_attributes_to_program(&shader_program, 1, "uv");
        shader_program
    }

    pub unsafe fn set_focal_offset(&self, offset: &Vec2) {
        gl::Uniform2f(self.focal_offset_location, offset.x, offset.y);
    }

    pub unsafe fn set_aspect_ratio(&self, aspect_ratio: f32) {
        gl::Uniform1f(self.aspect_ratio_location, aspect_ratio);
    }

    pub unsafe fn set_num_lights(&self, num_lights: i32) {
        gl::Uniform1i(self.num_lights_location, num_lights);
    }

    pub unsafe fn set_light_positions(&self, num_lights: i32, light_positions: &Vec<f32>) {
        gl::Uniform2fv(
            self.light_positions_location,
            num_lights,
            light_positions.as_ptr(),
        );
    }
}

impl ShaderProgram for PostprocessShader {
    fn get_id(&self) -> GLuint {
        self.id
    }

    unsafe fn start(&self) {
        shader_handler::start_program(self)
    }

    unsafe fn stop(&self) {
        shader_handler::stop_program()
    }

    fn get_parameter_schema(&self) -> ParameterSchema {
        ParameterSchema {
            required_params: vec![
                ("uFocalOffset".to_string(), ShaderParameterType::Vec2),
                ("uAspectRatio".to_string(), ShaderParameterType::Float),
                ("uNumLights".to_string(), ShaderParameterType::Int),
                (
                    "uLightPositions".to_string(),
                    ShaderParameterType::Vec2Array,
                ),
            ],
        }
    }
}
