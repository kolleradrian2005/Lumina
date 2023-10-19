use std::ffi::CString;

use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::texture_handler::TextureHandler;
use crate::scene::Scene;

pub struct PlayerRenderer {
   shader: ShaderProgram
}

impl PlayerRenderer {
    pub fn init() -> Self {
        let fragment_shader: Shader = Shader::new("player.frag", gl::FRAGMENT_SHADER);
        let vertex_shader: Shader = Shader::new("player.vert", gl::VERTEX_SHADER);
        let shader_program: ShaderProgram = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        shader_program.bind_attributes(1, "uv");
        PlayerRenderer {
            shader: shader_program
        }
    }

    pub fn render(&self, scene: &mut Scene, texture_handler: &mut TextureHandler) {
        self.shader.start();
        unsafe {
            let model_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()));
            let view_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uViewMatrix").unwrap()));
            let projection_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uProjectionMatrix").unwrap()));
            let view_matrix = scene.camera.get_view_matrix();
            let projection_matrix = scene.camera.get_projection_matrix();
            let has_texture_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uHasTexture").unwrap()));
            let color_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uColor").unwrap()));
            let flipped_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uFlipped").unwrap()));
            let model_matrix = scene.player.get_translation_matrix();
            let model = &mut scene.player.model;
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_ptr() as * const f32);
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view_matrix.as_ptr() as * const f32);
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_matrix.as_ptr() as * const f32);
            gl::Uniform1i(flipped_location, model.is_flipped() as i32);
            gl::BindVertexArray(model.get_vao());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
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
            gl::DrawElements(
                gl::TRIANGLES,
                model.get_vertex_count(),
                gl::UNSIGNED_INT,
                0 as * const _
            );
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::BindVertexArray(0);
        };
        self.shader.stop();
    }
}
