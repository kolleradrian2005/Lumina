use std::ffi::CString;

use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::scene::Scene;

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

    pub fn render(&self, scene: &mut Scene) {
        self.shader.start();
        unsafe {
            for model in scene.terrain.get_loaded_tiles() {
                //let x: f32 = raw_x as f32/ resolution as f32;
                //let focal_offset = &scene.focal_offset;

                let model_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uModelMatrix").unwrap()));
                let view_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uViewMatrix").unwrap()));
                let projection_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uProjectionMatrix").unwrap()));
                
                let view_matrix = scene.camera.get_view_matrix();
                let projection_matrix = scene.camera.get_projection_matrix();
                let model_matrix = model.get_model_matrix();

                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, model_matrix.as_ptr() as * const f32);
                gl::UniformMatrix4fv(view_location, 1, gl::FALSE, view_matrix.as_ptr() as * const f32);
                gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection_matrix.as_ptr() as * const f32);

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
