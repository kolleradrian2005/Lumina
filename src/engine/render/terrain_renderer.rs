use std::ffi::CString;

use crate::frame_buffer::Framebuffer;
use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::texture_handler::TextureHandler;
use crate::{model::Model, scene::Scene};
use crate::{terrain, window_handler};

pub struct TerrainRenderer {
    shader: ShaderProgram,
}

impl TerrainRenderer {
    pub fn init() -> Self {
        let fragment_shader = Shader::new("terrain.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("terrain.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        shader_program.bind_attributes(1, "uv");
        TerrainRenderer {
            shader: shader_program,
        }
    }

    pub fn render(&self, scene: &mut Scene, framebuffer: &mut Framebuffer) {
        self.shader.start();
        // TODO: Use terrain.model OR fullscreen model
        let model = &mut framebuffer.model;
        unsafe {
            // Renders terrain parts each 1 x changes (Could be optimized)
            for x in -10..10 {
                let focal_offset = &scene.focal_offset;
                let height_location = gl::GetUniformLocation(
                    self.shader.id,
                    std::ffi::CStr::as_ptr(&CString::new("uHeight").unwrap()),
                );
                let x_location = gl::GetUniformLocation(
                    self.shader.id,
                    std::ffi::CStr::as_ptr(&CString::new("uX").unwrap()),
                );
                let playerpos_location = gl::GetUniformLocation(
                    self.shader.id,
                    std::ffi::CStr::as_ptr(&CString::new("uPlayerpos").unwrap()),
                );
                let aspect_ratio_location = gl::GetUniformLocation(
                    self.shader.id,
                    std::ffi::CStr::as_ptr(&CString::new("uAspectRatio").unwrap()),
                );
                let focal_offset_location = gl::GetUniformLocation(
                    self.shader.id,
                    std::ffi::CStr::as_ptr(&CString::new("uFocalOffset").unwrap()),
                );
                gl::Uniform1f(
                    aspect_ratio_location,
                    window_handler::WINDOW_WIDTH as f32 / window_handler::WINDOW_HEIGHT as f32,
                );
                gl::Uniform2f(
                    playerpos_location,
                    scene.player.get_position().x,
                    scene.player.get_position().y,
                );
                gl::Uniform1f(height_location, scene.terrain.get_height(x));
                gl::Uniform1ui(x_location, x as u32);
                gl::Uniform2f(focal_offset_location, focal_offset.x, focal_offset.y);
                gl::BindVertexArray(model.get_vao());
                gl::EnableVertexAttribArray(0);
                gl::EnableVertexAttribArray(1);
                gl::DrawElements(
                    gl::TRIANGLES,
                    model.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    0 as *const _,
                );
                gl::DisableVertexAttribArray(0);
                gl::DisableVertexAttribArray(1);

                gl::BindVertexArray(0);
            }
        };
        self.shader.stop();
    }
}
