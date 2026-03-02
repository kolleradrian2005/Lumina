use std::{collections::HashMap, sync::Arc};

use crate::{
    math::{transformation, vec3::Vec3},
    model::{mesh::Mesh, wireframe},
    render::render_entity::RenderEntity,
    scene::world::{
        component::{
            collider_component::{ColliderComponent, ColliderShape},
            material_component::{DrawMode, MaterialComponent},
            transform_component::TransformComponent,
        },
        system::system::System,
        world::World,
    },
    texture::{
        resource_manager::ResourceManager,
        resource_provider::ResourceProvider,
        texture::{StaticColor, Texture},
    },
};

#[derive(Hash, PartialEq, Eq, Clone)]
enum ShapeKey {
    Capsule2D(u32, u32),
    Rect(u32, u32),
}

impl ShapeKey {
    fn from_shape(shape: &ColliderShape) -> Self {
        match shape {
            ColliderShape::Capsule2D { width, height } => {
                ShapeKey::Capsule2D((*width * 100.0) as u32, (*height * 100.0) as u32)
            }
            ColliderShape::Rect { width, height } => {
                ShapeKey::Rect((*width * 100.0) as u32, (*height * 100.0) as u32)
            }
        }
    }
}

pub struct DebugColliderSystem {
    collider_meshes: HashMap<ShapeKey, Arc<Mesh>>,
}

impl DebugColliderSystem {
    pub fn new() -> Self {
        Self {
            collider_meshes: HashMap::new(),
        }
    }
}

impl System for DebugColliderSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (collider, transform)) in
            world.query_mut::<(&mut ColliderComponent, &mut TransformComponent)>()
        {
            if let Some(resource_manager) = world.get_resource_mut::<ResourceManager>() {
                let mesh = self.get_or_create_mesh(resource_manager, &collider.shape);
                let debug_shader = resource_manager.get_shader("debug_shader");
                let material = MaterialComponent::new(
                    Texture::StaticColor(StaticColor::new((1.0, 0.0, 0.0).into())),
                    debug_shader,
                )
                .with_draw_mode(DrawMode::Lines);
                /*let shader = resource_manager.get_shader("model");
                let material = MaterialComponent::new(
                    Texture::StaticColor(StaticColor::new((1.0, 0.0, 0.0).into())),
                    shader,
                );*/
                let mut transform = transform.clone();
                transform.position += Vec3::from_vec2(collider.offset, 0.0);
                world.render_packet.entities.push(RenderEntity {
                    mesh,
                    material,
                    is_flipped: transform.is_flipped,
                    transform_matrix: transformation::create_transform_matrix(&transform, None), // TODO: maybe check for parent components in the future
                });
            }
        }
    }
}

impl DebugColliderSystem {
    fn get_or_create_mesh(
        &mut self,
        resource_manager: &mut ResourceManager,
        shape: &ColliderShape,
    ) -> Arc<Mesh> {
        let key = ShapeKey::from_shape(shape);
        self.collider_meshes
            .entry(key)
            .or_insert_with(|| {
                let (vertices, indices, uvs) = match shape {
                    ColliderShape::Capsule2D { width, height } => {
                        wireframe::capsule(*width, *height, 16)
                    }
                    ColliderShape::Rect { width, height } => wireframe::rectangle(*width, *height),
                };
                let mesh = resource_manager.load_mesh(vertices, indices, uvs);
                Arc::new(mesh.expect("Unable to load collider mesh"))
            })
            .clone()
    }
}
