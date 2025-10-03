use include_assets::NamedArchive;

use crate::engine::{
    math::transformation,
    scene::{background::Background, scene::Scene},
    shader::{background_shader::BackgroundShader, shader_program::ShaderProgram},
    texture::texture::Texture,
};

pub struct BackgroundRenderer {
    shader: BackgroundShader,
}

impl BackgroundRenderer {
    pub fn init(archive: &NamedArchive) -> Self {
        unsafe {
            BackgroundRenderer {
                shader: BackgroundShader::new(archive),
            }
        }
    }

    pub unsafe fn render(&self, scene: &Scene) {
        self.shader.start();

        let background = scene.get_world().expect_resource::<Background>();
        let layers = &background.layers;
        for i in 0..layers.len() {
            let model = layers.get(i).unwrap();
            let mesh = model.get_mesh();
            let model_matrix = transformation::create_model_matrix(model, None);
            self.shader.set_model_matrix(&model_matrix);
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
                Texture::GradientTexture(gradient_texture) => {
                    self.shader.set_color1(gradient_texture.color1);
                    self.shader.set_color2(gradient_texture.color2);
                }
            }
            self.shader.set_has_texture(texture.has_texture());
            self.shader.set_flipped(model.is_flipped());
            gl::BindVertexArray(mesh.get_vao());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.get_vertex_count(),
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
