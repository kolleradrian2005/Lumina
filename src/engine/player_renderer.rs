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
        let fragment_shader = Shader::new("./assets/shaders/player.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("./assets/shaders/player.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
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
            let model_matrix = &scene.player.get_translation_matrix();
            let model = &mut scene.player.model;
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_ptr() as * const f32);
            gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view_matrix.as_ptr() as * const f32);
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_matrix.as_ptr() as * const f32);
            gl::BindVertexArray(model.get_vao());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            if model.has_texture() {
                let texture_name = model.get_current_texture();
                let texture = texture_handler.get_texture(texture_name.unwrap());
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
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
