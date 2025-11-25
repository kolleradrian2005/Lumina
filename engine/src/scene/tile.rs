use noise::Perlin;

use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    model::sprite,
    render::{renderable::MeshLoadState, scene_renderer::ObjectType},
    scene::world::{
        component::{
            current_component::CurrentComponent,
            model_component::ModelComponent,
            shader_params_component::{ShaderParam, ShaderParamsComponent},
            texture_component::TextureComponent,
            transform_component::TransformComponent,
        },
        entity::entity::Entity,
        world::World,
    },
    texture::{resource_provider::ResourceProvider, texture::Texture},
    transformable::Transformable,
};

use super::terrain::Terrain;

pub struct Tile {
    entity: Entity,
    uphill: bool,
    height: f32, // Height difference to be exact
    objects: Vec<Entity>,
}

impl Tile {
    pub fn generate(
        world: &mut World,
        size: f32,
        x: i32,
        noise: &Perlin,
        texture: &Texture,
        resource_provider: &mut dyn ResourceProvider,
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
                let seaweed = world.create_entity();
                let seaweed_model = resource_provider.get_model("seagrass");
                if let Texture::StaticTexture(texture) = seaweed_model.get_texture() {
                    position.y +=
                        seaweed_model.get_scale().y * texture.get_normalized_dimensions().1 / 2.0;
                }
                world.add_component::<CurrentComponent>(seaweed, CurrentComponent::default());
                world.add_component::<ModelComponent>(
                    seaweed,
                    ModelComponent {
                        mesh: MeshLoadState::Loaded(seaweed_model.get_mesh().clone()),
                        object_type: ObjectType::SeaGrass,
                    },
                );
                world.add_component::<TextureComponent>(
                    seaweed,
                    seaweed_model.get_texture().clone().into(),
                );
                world.add_component(
                    seaweed,
                    TransformComponent {
                        position: position,
                        scale: seaweed_model.get_scale(),
                        rotation: 0.0,
                        is_flipped: false,
                    },
                );
                world.add_component(
                    seaweed,
                    ShaderParamsComponent {
                        params: vec![ShaderParam::Current(0f32)].into(),
                    },
                );
                objects.push(seaweed);
            }
        }
        let tile = world.create_entity();
        world.add_component::<ModelComponent>(
            tile,
            ModelComponent {
                mesh: MeshLoadState::CreateRequest {
                    vertices: vertices.into(),
                    indices: sprite::INDICES.into(),
                    uvs: sprite::UVS.into(),
                }
                .into(),
                object_type: ObjectType::Terrain,
            },
        );
        world.add_component::<TextureComponent>(tile, texture.clone().into());
        world.add_component(
            tile,
            TransformComponent {
                position: tile_position,
                scale: Vec2::uniform(1.0),
                rotation: 0.0,
                is_flipped: false,
            },
        );
        world.add_component(
            tile,
            ShaderParamsComponent {
                params: vec![
                    ShaderParam::IsUphill(uphill),
                    ShaderParam::Height(top - bot),
                ]
                .into(),
            },
        );
        Tile {
            entity: tile,
            uphill,
            height: top - bot,
            objects,
        }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn is_uphill(&self) -> bool {
        self.uphill
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_objects(&self) -> &Vec<Entity> {
        &self.objects
    }
}
