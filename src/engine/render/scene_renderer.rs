use include_assets::NamedArchive;

use crate::engine::math::transformation;
use crate::engine::model::model::Model;
use crate::engine::model::model_group::ModelGroup;
use crate::engine::scene::scene::Scene;
use crate::engine::scene::tile::Tile;
use crate::engine::shader::model_shader::ModelShader;
use crate::engine::shader::shader_program::ShaderProgram;
use crate::engine::texture::texture::Texture;
use crate::engine::transformable::Transformable;

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

    unsafe fn render_tile(&self, shader: &ModelShader, tile: &Tile) {
        shader.set_isuphill(tile.is_uphill());
        shader.set_height(tile.get_height());
        self.render_model(shader, tile.get_model(), None);
    }

    unsafe fn render_model_group(&self, shader: &ModelShader, model_group: &ModelGroup) {
        for model in model_group.get_models() {
            let model_matrix = transformation::create_model_matrix(model, Some(model_group));
            self.render_model(shader, model, Some(model_matrix));
        }
    }

    unsafe fn render_model(
        &self,
        shader: &ModelShader,
        model: &Model,
        model_matrix_opt: Option<[[f32; 4]; 4]>,
    ) {
        let model_matrix =
            model_matrix_opt.unwrap_or(transformation::create_model_matrix(model, None));
        shader.set_model_matrix(model_matrix);
        shader.set_flipped(model.is_flipped());
        gl::BindVertexArray(model.get_vao());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        let texture = model.get_texture();
        match texture {
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
        shader.set_texture_type(&texture);
        gl::DrawElements(
            match shader.has_tesselation() {
                true => gl::PATCHES,
                false => gl::TRIANGLES,
            },
            model.get_vertex_count(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }

    pub unsafe fn render(&self, scene: &mut Scene) {
        /* TESSELATION ENABLED */

        let mut current_shader = match &self.shader_with_tesselation {
            Some(x) => x,
            None => &self.shader_without_tesselation,
        };
        current_shader.start();

        // Render seagrass
        current_shader.set_object_type(ObjectType::SeaGrass);

        for tile in scene.get_world().get_terrain().get_tiles() {
            for object in tile.get_objects() {
                let object_position = object.get_position();
                let mut current = scene
                    .get_world()
                    .get_water()
                    .get_current(&object_position.xy());

                let player_position = scene.player.model_group.get_position();
                let player_distance = (object_position - player_position).length();
                if player_distance != 0.0 {
                    let mut influence = 1.0 / (player_distance.powf(1.5) * 10.0);
                    let influence_treshold = 5.5;
                    influence = f32::min(influence_treshold, influence);
                    current += influence * scene.player.get_velocity().x;
                }

                current_shader.set_current(current);
                self.render_model(current_shader, object, None);
            }
        }
        current_shader.stop();

        /* TESSELATION DISABLED */
        current_shader = &self.shader_without_tesselation;
        current_shader.start();

        current_shader.set_object_type(ObjectType::Terrain);

        for tile in scene.get_world_mut().get_terrain_mut().get_tiles_mut() {
            tile.prepare_model();
            self.render_tile(&current_shader, tile);
        }

        current_shader.set_object_type(ObjectType::Default);

        // Render each model of the scene
        for model in scene.models.iter() {
            self.render_model(current_shader, model, None)
        }

        current_shader.set_object_type(ObjectType::Default);

        // Render player
        self.render_model_group(current_shader, &scene.player.model_group);
        // Render particles
        for particle_ptr in scene.get_particles().iter() {
            let particle_system = particle_ptr.read().unwrap();
            for particle in particle_system.particles.iter() {
                self.render_model(current_shader, particle.get_model(), None);
            }
        }

        gl::BindVertexArray(0);

        current_shader.stop();
    }
}
