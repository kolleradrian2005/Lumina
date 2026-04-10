use lumina_engine::{
    logic::{
        ecs::{
            component::{material::Material, model::Model, transform::Transform},
            entity::entity::Entity,
        },
        scene::world::World,
    },
    math::{vec2::Vec2, vec3::Vec3},
    render::resource::{
        resource_manager::ResourceManager,
        resource_provider::ResourceProvider,
        texture::texture::{StaticColor, Texture},
    },
    spawn_entity,
};

use super::fish::Fish;

pub struct FishPrefab;

impl FishPrefab {
    pub fn spawn(world: &mut World) -> Entity {
        let resource_manager: &mut ResourceManager = world.expect_resource_mut::<ResourceManager>();
        let shader = resource_manager.get_shader("model").clone();
        let model = Model::from(resource_manager.get_mesh("square"));
        let texture = resource_manager.load_static_texture("fish.png");
        spawn_entity!(
            world,
            Transform {
                position: Vec3::new(0.0, 0.0, 0.0),
                rotation: 0.0,
                scale: Vec2::new(0.04, 0.04),
                is_flipped: false,
            },
            model,
            Material::new(
                texture.unwrap_or_else(|| Texture::StaticColor(StaticColor::new(
                    (0.5, 0.5, 0.5).into()
                ))),
                shader,
            ),
            Fish { speed: 0.07 }
        )
    }
}
