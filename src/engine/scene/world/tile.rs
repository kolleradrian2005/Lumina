use std::sync::Arc;

use noise::Perlin;

use crate::engine::{
    command_queue::CommandQueue,
    math::vec3::Vec3,
    model::{model::Model, sprite},
    texture::{resource_manager::ResourceManager, texture::Texture},
    transformable::Transformable,
};

use super::terrain::Terrain;

#[derive(Clone)]
enum TileModel {
    Model(Model),
    RawModel {
        vertices: [f32; 12],
        texture: Texture,
        position: Vec3,
    },
}

pub struct Tile {
    model: TileModel,
    uphill: bool,
    height: f32, // Height difference to be exact
    objects: Vec<Model>,
}

impl Tile {
    pub fn generate(
        size: f32,
        x: i32,
        noise: &Perlin,
        texture: &Texture,
        resource_manager: &mut ResourceManager,
    ) -> Tile {
        // Generate raw model

        let z_index = 0.0;
        let previous_y: f32 = Terrain::get_height_noise(x - 1, noise);
        let current_y: f32 = Terrain::get_height_noise(x, noise);
        let next_y: f32 = Terrain::get_height_noise(x + 1, noise);

        let left_y_offset = (previous_y - current_y) / 2.0;
        let right_y_offset = (current_y - next_y) / 2.0;
        let a = size / 2.0 + left_y_offset;
        let b = size / 2.0 - right_y_offset;
        let uphill = a < b;
        let top = f32::max(previous_y, next_y);
        let bot = f32::min(previous_y, next_y);

        let height = top - bot;

        let vertices: [f32; 12] = [
            // Bottom left
            -size / 2.0,
            bot,
            sprite::Z_DEFAULT,
            // Bottom right
            size / 2.0,
            bot,
            sprite::Z_DEFAULT,
            // Top right
            size / 2.0,
            top,
            sprite::Z_DEFAULT,
            // Top left
            -size / 2.0,
            top,
            sprite::Z_DEFAULT,
        ];
        let tile_position = Vec3::new(x as f32 * size, current_y, z_index);

        // Generate objects
        let mut objects = Vec::new();

        // Generate seaweed
        let seaweed_per_chunk = 20;
        for i in 0..seaweed_per_chunk {
            let seaweed_noise = Terrain::get_seaweed_noise(x * seaweed_per_chunk + i, noise);
            let seaweed_treshold = 1.25;
            if seaweed_treshold < seaweed_noise.abs() {
                let ratio = i as f32 / seaweed_per_chunk as f32;
                let x = tile_position.x - size / 2.0 + size * ratio;
                let mut position = Vec3::new(
                    x,
                    current_y
                        + bot
                        + height
                            * Terrain::interpolate(f32::from(!uphill), f32::from(uphill), ratio),
                    z_index,
                );
                position.z = z_index;
                let mut seaweed = resource_manager.get_model("seagrass");
                if let Texture::StaticTexture(texture) = seaweed.get_texture() {
                    position.y +=
                        seaweed.get_scale().y * texture.get_normalized_dimensions().1 / 2.0;
                }
                seaweed.set_position(position);
                objects.push(seaweed);
            }
        }
        Tile {
            model: TileModel::RawModel {
                vertices,
                texture: texture.clone(),
                position: tile_position,
            },
            uphill,
            height: top - bot,
            objects,
        }
    }

    pub fn prepare_model(&mut self, command_queue: Arc<CommandQueue>) {
        if let TileModel::RawModel {
            vertices,
            texture,
            position,
        } = self.model.clone()
        {
            let mut model = Model::new(command_queue, &vertices, &sprite::INDICES, &sprite::UVS);
            model.set_texture(texture);
            model.set_position(position);
            self.model = TileModel::Model(model);
        }
    }

    pub fn get_model(&self) -> &Model {
        match &self.model {
            TileModel::Model(model) => model,
            _ => panic!("Used tile model before preparation"),
        }
    }

    pub fn is_uphill(&self) -> bool {
        self.uphill
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_objects(&self) -> &Vec<Model> {
        &self.objects
    }
}
