use gl::types::*;
use crate::{scene::Scene, model::Model};

pub struct SceneRenderer {
    // TODO: shaders
}

impl SceneRenderer {
    pub fn init() -> Self {
        SceneRenderer { }
    }

    pub fn render(&self, scene: &Scene) {
        let models: &Vec<Model> = scene.get_models();
        for model in models.iter() {
            unsafe {
                gl::BindVertexArray(model.get_vao());
                gl::EnableVertexAttribArray(0);
                gl::DrawElements(gl::TRIANGLES, model.get_vertex_count(), gl::UNSIGNED_INT, std::ptr::null());
                gl::DisableVertexAttribArray(0);
            };
        }
        unsafe {
            gl::BindVertexArray(0)
        };
    }
}