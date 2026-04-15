use super::system::System;
use crate::{
    logic::{
        ecs::component::collider::{Collider, ColliderShape},
        scene::{debug_config::DebugConfig, world::World},
    },
    render::{
        model::wireframe,
        resource::resource_manager::{ColliderShapeKey, ResourceManager},
    },
    shared::input::input_state::InputState,
};
use std::sync::Arc;
use winit::keyboard::{Key, NamedKey};

pub struct DebugSystem {
    was_pressed: bool,
}

impl DebugSystem {
    pub fn new() -> Self {
        Self { was_pressed: false }
    }
}

impl System for DebugSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        if let Some(input) = world.get_resource::<InputState>() {
            let pressed = input.is_pressed(Key::Named(NamedKey::F3));
            if pressed && !self.was_pressed {
                if let Some(config) = world.get_resource_mut::<DebugConfig>() {
                    config.enabled = !config.enabled;
                    log::info!(
                        "Debug rendering {}",
                        match config.enabled {
                            true => "enabled",
                            false => "disabled",
                        }
                    );
                }
            }
            self.was_pressed = pressed;
        }

        let enabled = world
            .get_resource::<DebugConfig>()
            .map_or(false, |c| c.enabled);
        if !enabled {
            return;
        }

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
