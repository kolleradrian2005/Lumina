use gl::types::*;
use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::{scene::Scene, model::Model};

pub struct SceneRenderer {
   shader: ShaderProgram
}

impl SceneRenderer {
    pub fn init() -> Self {
        let fragment_shader = Shader::new("./assets/shaders/draw.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("./assets/shaders/draw.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        SceneRenderer {
            shader: shader_program
        }
    }

    pub fn render(&self, scene: &Scene) {
        self.shader.start();
        let models: &Vec<Model> = scene.get_models();
        for model in models.iter() {
            unsafe {
                gl::BindVertexArray(model.get_vao());
                gl::EnableVertexAttribArray(0);
                gl::DrawElements(
                    gl::TRIANGLES,
                    model.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    0 as *const _
                );
                gl::DisableVertexAttribArray(0);
            };
        }
        unsafe {
            gl::BindVertexArray(0)
        };
        self.shader.stop();
    }
}