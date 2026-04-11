use lumina_engine::{
    logic::{
        ecs::{
            component::{
                collider::{Collider, ColliderShape},
                force::{AppliedForce, Force, ForceEffect, ForceMode},
                material::Material,
                model::Model,
                movement::Movement,
                transform::Transform,
            },
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

use crate::{scene::water::Water, sea_trash::sea_trash::SeaTrash};

pub struct BottleTrashPrefab;

impl BottleTrashPrefab {
    pub fn spawn(world: &mut World) -> Entity {
        let resource_manager: &mut ResourceManager = world.expect_resource_mut::<ResourceManager>();
        let shader = resource_manager.get_shader("model").clone();
        let model = Model::from(resource_manager.get_mesh("square"));
        let texture = resource_manager.load_static_texture("bottle_trash.png");
        let mut force_component = Force::new(0.1);
        force_component.apply_force(AppliedForce {
            id: "water_resistance".to_string(),
            effect: ForceEffect::Drag(world.expect_resource::<Water>().get_resistance()),
            mode: ForceMode::Continuous,
        });
        force_component.apply_force(AppliedForce {
            id: "gravity".to_string(),
            effect: ForceEffect::Linear(Vec3::new(0.0, -0.005, 0.0)),
            mode: ForceMode::Continuous,
        });
        spawn_entity!(
            world,
            Transform {
                position: Vec3::new(0.0, 0.0, 0.0),
                rotation: 0.0,
                scale: Vec2::new(0.06, 0.06),
                is_flipped: false,
            },
                Collider::new(ColliderShape::Capsule2D {
                    width: 0.3,
                    height: 0.9
                }),
            force_component,
            Movement::default(),
            model,
            Material::new(
                texture.unwrap_or_else(|| Texture::StaticColor(StaticColor::new(
                    (0.5, 0.5, 0.5).into()
                ))),
                shader,
            ),
            SeaTrash
        )
    }
}
