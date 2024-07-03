use include_assets::NamedArchive;

use crate::engine::{
    gui::gui_manager::GuiManager,
    math::transformation,
    shader::{gui_shader::GuiShader, shader_program::ShaderProgram},
    texture::texture::Texture,
};

pub struct GuiRenderer {
    shader: GuiShader,
}

impl GuiRenderer {
    pub fn init(archive: &NamedArchive) -> Self {
        unsafe {
            GuiRenderer {
                shader: GuiShader::new(archive),
            }
        }
    }

    pub unsafe fn render(&self, gui_manager: &GuiManager, aspect_ratio: f32) {
        self.shader.start();
        for model in gui_manager.get_elements() {
            let model_matrix = transformation::create_model_matrix(&model, None);
            self.shader.set_model_matrix(model_matrix);
            self.shader.set_aspect_ratio(aspect_ratio);

            let texture = model.get_texture();
            match texture {
                Texture::StaticColor(static_color) => {
                    self.shader.set_color(static_color.color);
                }
                Texture::StaticTexture(static_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, static_texture.get_id());
                }
                Texture::AnimatedTexture(animated_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, animated_texture.current_texture().get_id());
                }
                Texture::GradientTexture(_) => {}
            }
            self.shader.set_has_texture(texture.has_texture());
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
        self.shader.stop();
    }
}
