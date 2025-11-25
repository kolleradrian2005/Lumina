use include_assets::NamedArchive;


use crate::scene::scene::Scene;

use crate::shader::model_shader::ModelShader;
use crate::shader::shader_program::ShaderProgram;
use crate::texture::texture::Texture;

use super::renderable::Renderable;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectType {
    Default,
    Terrain,
    SeaGrass,
}

pub struct SceneRenderer {
    shader_without_tesselation: ModelShader,
    shader_with_tesselation: Option<ModelShader>,
}

impl SceneRenderer {
    pub fn init(archive: &NamedArchive) -> Self {
        unsafe {
            // Tesselation shaders are not supported on gles 3
            let shader_with_tesselation =
                cfg!(not(target_os = "android")).then(|| ModelShader::new(archive, true));
            SceneRenderer {
                shader_without_tesselation: ModelShader::new(archive, false),
                shader_with_tesselation: shader_with_tesselation,
            }
        }
    }

    pub unsafe fn render(&self, scene: &mut Scene) {
        /* TESSELATION ENABLED */

        let mut current_shader = self
            .shader_with_tesselation
            .as_ref()
            .unwrap_or(&self.shader_without_tesselation);
        current_shader.start();

        // Render seagrass
        current_shader.set_object_type(ObjectType::SeaGrass);

        for renderable in scene.get_world().renderables.iter() {
            if let ObjectType::SeaGrass = renderable.object_type {
                self.render_entity(renderable, current_shader);
            }
        }

        current_shader.stop();

        /* TESSELATION DISABLED */
        current_shader = &self.shader_without_tesselation;
        current_shader.start();

        current_shader.set_object_type(ObjectType::Default);

        let mut current_type = ObjectType::Default;
        for renderable in scene.get_world().renderables.iter() {
            if let ObjectType::SeaGrass = renderable.object_type {
                continue;
            }
            if current_type != renderable.object_type {
                current_type = renderable.object_type;
                current_shader.set_object_type(current_type);
            }
            self.render_entity(renderable, current_shader);
        }

        gl::BindVertexArray(0);

        current_shader.stop();
    }

    pub unsafe fn render_entity(&self, renderable: &Renderable, shader: &ModelShader) {
        let mesh = match &renderable.mesh {
            Some(mesh) => mesh,
            None => return,
        };
        if let Some(shader_params) = &renderable.shader_params {
            shader.set_shader_params(&shader_params.params);
        }
        shader.set_model_matrix(renderable.transform_matrix);
        shader.set_flipped(renderable.is_flipped);
        gl::BindVertexArray(mesh.get_vao());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        match &renderable.texture {
            Texture::StaticColor(static_color) => {
                shader.set_color(static_color.color);
            }
            Texture::StaticTexture(static_texture) => {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, static_texture.get_id());
            }
            Texture::AnimatedTexture(animated_texture) => {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, animated_texture.current_texture().get_id());
            }
            Texture::GradientTexture(_) => {} // Not implemented
        }
        shader.set_texture_type(&renderable.texture);
        gl::DrawElements(
            match shader.has_tesselation() {
                true => gl::PATCHES,
                false => gl::TRIANGLES,
            },
            mesh.get_vertex_count(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }
}
