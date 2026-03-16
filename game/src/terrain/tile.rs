use lumina_engine::{
    logic::{
        ecs::{
            component::{
                material::{DrawMode, Material},
                model::Model,
                transform::Transform,
            },
            entity::entity::Entity,
        },
        scene::world::World,
    },
    math::{vec2::Vec2, vec3::Vec3},
    render::{
        model::sprite,
        resource::{
            resource_manager::ResourceManager, resource_provider::ResourceProvider,
            texture::Texture,
        },
    },
};
use noise::Perlin;

use crate::components::current::Current;
use crate::object_type::ObjectType;

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
                // TODO: use prefab
                let seaweed = world.create_entity();
                let seaweed_mesh = resource_manager.get_mesh("seagrass");
                let shader = resource_manager.get_shader("model_with_tesselation");
                let use_tesselation = shader.get_handle().has_tesselation;
                if let Some(Texture::StaticTexture(texture)) =
                    resource_manager.load_static_texture("seagrass0.png")
                {
                    position.y += 0.08 * texture.get_normalized_dimensions().1 / 2.0;
                    let mut material = Material::new(Texture::StaticTexture(texture), shader)
                        .with_param("uObjectType", ObjectType::SeaGrass as i32);
                    if use_tesselation {
                        material.set_param("uCurrent", 0f32);
                    }
                    world.add_component(seaweed, material);
                }
                world.add_component::<Current>(seaweed, Current::default());
                world.add_component::<Model>(
                    seaweed,
                    Model {
                        mesh: seaweed_mesh,
                        //object_type: ObjectType::SeaGrass,
                    },
                );

                world.add_component(
                    seaweed,
                    Transform {
                        position: position,
                        scale: Vec2::uniform(0.08),
                        rotation: 0.0,
                        is_flipped: false,
                    },
                );
                objects.push(seaweed);
            }
        }
        let tile = world.create_entity();
        let mesh = resource_manager
            .load_mesh(
                vertices.to_vec(),
                sprite::INDICES.to_vec(),
                sprite::UVS.to_vec(),
            )
            .unwrap();
        world.add_component::<Model>(
            tile,
            Model {
                mesh: mesh.into(),
                //object_type: ObjectType::Terrain,
            },
        );
        let shader = resource_manager.get_shader("model");
        let material = Material::new(texture.clone(), shader.clone())
            .with_param("uObjectType", ObjectType::Terrain as i32)
            .with_param("uTerrainIsUphill", uphill)
            .with_param("uTerrainHeight", top - bot)
            .with_draw_mode(match shader.get_handle().has_tesselation {
                true => DrawMode::Patches,
                false => DrawMode::Triangles,
            });

        world.add_component(tile, material);
        world.add_component(
            tile,
            Transform {
                position: tile_position,
                scale: Vec2::uniform(1.0),
                rotation: 0.0,
                is_flipped: false,
            },
        );
        /*world.add_component(
            tile,
            ShaderParamsComponent {
                params: vec![
                    ShaderParam::IsUphill(uphill),
                    ShaderParam::Height(top - bot),
                ]
                .into(),
            },
        );*/
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
