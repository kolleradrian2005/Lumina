use std::borrow::BorrowMut;
use std::ffi::CString;

use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::texture_handler::TextureHandler;
use crate::scene::Scene;

pub struct BackgroundRenderer {
   shader: ShaderProgram
}

impl BackgroundRenderer {
    pub fn init() -> Self {
        let fragment_shader = Shader::new("background.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("background.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        shader_program.bind_attributes(1, "uv");
        BackgroundRenderer {
            shader: shader_program
        }
    }

    pub fn render(&self, scene: &mut Scene, texture_handler: &mut TextureHandler) {
        self.shader.start();
        let layers = &mut scene.background.layers;
        unsafe {
            for i in 0..layers.len() {
                let mut model = layers.get_mut(i).unwrap();
                let color_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uColor").unwrap()));
                //let layer_index_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uLayerIndex").unwrap()));
                let has_texture_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uHasTexture").unwrap()));
                let texture_name = model.get_current_texture();
                let use_texture = !texture_name.is_none(); 
                if use_texture {
                    let texture = texture_handler.get_texture(texture_name.unwrap());
                    gl::Uniform1f(has_texture_location, 1.0);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
                } else {
                    gl::Uniform1f(has_texture_location, 0.0);
                    let color = model.get_color();
                    gl::Uniform3f(color_location, color.x, color.y, color.z);
                }
                //gl::Uniform1i(layer_index_location, i as i32);
                gl::BindVertexArray(model.get_vao());
                gl::EnableVertexAttribArray(0);
                if model.has_texture() {
                    gl::EnableVertexAttribArray(1);
                }
                //gl::DrawArrays(gl::TRIANGLES, 0, 6);
                gl::DrawElements(
                    gl::TRIANGLES,
                    model.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    0 as * const _
                );
                gl::DisableVertexAttribArray(0);
                if model.has_texture() {
                    gl::DisableVertexAttribArray(1);
                }
                gl::BindVertexArray(0);
            }
        };
        self.shader.stop();
    }
}
