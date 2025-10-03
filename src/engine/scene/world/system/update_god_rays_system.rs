use std::{
    sync::{Arc, Mutex},
};

use crate::engine::{
    math::vec3::Vec3,
    scene::{
        foreground::Foreground,
        world::{
            component::{
                camera_component::CameraComponent,
                transform_component::TransformComponent,
            },
            world::World,
        },
    },
};

use super::system::System;

pub struct UpdateGodRaysSystem;

impl System for UpdateGodRaysSystem {
    fn run(&self, world: &mut World, _: f32) {
        let player_position: Option<Vec3> = world
            .query_mut::<(&mut CameraComponent, &mut TransformComponent)>()
            .last()
            .map(|(_, (_, transform))| transform.position.clone());

        if let Some(player_position) = player_position {
            if let Ok(foreground) = &mut world.expect_resource::<Arc<Mutex<Foreground>>>().lock() {
                let noise_index = (player_position.x / foreground.god_rays_min_distance) as i32;
                let difference = foreground.loaded_noise_index - noise_index;
                if difference != 0 {
                    foreground.loaded_noise_index = noise_index;
                    if 0 < difference {
                        let x = foreground.loaded_noise_index - foreground.god_rays_max_count / 2;
                        let noise_value = foreground.get_noise_value(x);
                        foreground.god_rays_noise.push_front(noise_value);
                        foreground.god_rays_noise.pop_back();
                    } else {
                        let x = foreground.loaded_noise_index + foreground.god_rays_max_count / 2;
                        let noise_value = foreground.get_noise_value(x);
                        foreground.god_rays_noise.push_back(noise_value);
                        foreground.god_rays_noise.pop_front();
                    }
                }
            }
        }
    }
}
