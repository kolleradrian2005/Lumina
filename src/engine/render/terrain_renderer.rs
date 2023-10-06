use std::ffi::CString;

use crate::frame_buffer::Framebuffer;
use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::terrain;
use crate::texture_handler::TextureHandler;
use crate::{scene::Scene, model::Model};

pub struct TerrainRenderer {
   shader: ShaderProgram
}

impl TerrainRenderer {
    pub fn init() -> Self {
        let fragment_shader = Shader::new("terrain.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("terrain.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        shader_program.bind_attributes(1, "uv");
        TerrainRenderer {
            shader: shader_program
        }
    }

    pub fn render(&self, scene: &mut Scene, framebuffer: &mut Framebuffer) {
        self.shader.start();
        // TODO: Use terrain.model OR fullscreen model
        let model = &mut framebuffer.model;
        unsafe {
            // Renders terrain parts each 1 x changes (Could be optimized)
            for x in 0..5 {
                let height_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uHeight").unwrap()));
                let x_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uX").unwrap()));
                gl::Uniform1f(height_location, scene.terrain.get_height(x));
                gl::Uniform1ui(x_location, x as u32);
                gl::BindVertexArray(model.get_vao());
                gl::EnableVertexAttribArray(0);
                gl::EnableVertexAttribArray(1);
                gl::DrawElements(
                    gl::TRIANGLES,
                    model.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    0 as * const _
                );
                gl::DisableVertexAttribArray(0);
                gl::DisableVertexAttribArray(1);
                
                gl::BindVertexArray(0);
            }
        };
        self.shader.stop();
    }
}