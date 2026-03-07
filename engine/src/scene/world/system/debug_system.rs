use std::sync::Arc;

use crate::{
    model::wireframe,
    scene::world::{
        component::collider_component::{ColliderComponent, ColliderShape},
        system::system::System,
        world::World,
    },
    texture::resource_manager::{ColliderShapeKey, ResourceManager},
};

pub struct DebugSystem;

impl System for DebugSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (collider,)) in world.query::<(&ColliderComponent,)>() {
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
