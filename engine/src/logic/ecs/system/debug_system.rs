use super::system::System;
use crate::{
    logic::{
        ecs::component::collider::{Collider, ColliderShape},
        scene::world::World,
    },
    render::{
        model::wireframe,
        resource::resource_manager::{ColliderShapeKey, ResourceManager},
    },
};
use std::sync::Arc;
pub struct DebugSystem;

impl System for DebugSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (collider,)) in world.query::<(&Collider,)>() {
            if let Some(resource_manager) = world.get_resource_mut::<ResourceManager>() {
                let key = ColliderShapeKey::from_shape(&collider.shape);
                if resource_manager.get_collider_mesh(key.clone()).is_none() {
                    let (vertices, indices, uvs) = match collider.shape {
                        ColliderShape::Capsule2D { width, height } => {
                            wireframe::capsule(width, height, 16)
                        }
                        ColliderShape::Rect { width, height } => {
                            wireframe::rectangle(width, height)
                        }
                    };
                    let mesh = resource_manager.load_mesh(vertices, indices, uvs);
                    resource_manager.save_collider_mesh(
                        key,
                        Arc::new(mesh.expect("Unable to load collider mesh")),
                    );
                }
            }
        }
    }
}
