use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use lumina_engine::{
    math::vec3::Vec3,
    render::updatable::Updatable,
    scene::world::{
        component::{camera_component::CameraComponent, transform_component::TransformComponent},
        system::system::System,
        world::World,
    },
};

use crate::components::{
    follow_component::FollowComponent, player_state_component::PlayerStateComponent,
};

pub struct FollowSystem;

impl System for FollowSystem {
    fn run(&self, world: &mut World, delta_time: f32) {
        let updatables = world
            .expect_resource::<Arc<Mutex<VecDeque<Updatable>>>>()
            .clone();
        for (_, (camera, follow_component)) in
            world.query_mut::<(&mut CameraComponent, &mut FollowComponent)>()
        {
            let target_transform_component =
                world.get_component_mut::<TransformComponent>(follow_component.target_entity);
            let player_state_component =
                world.get_component::<PlayerStateComponent>(follow_component.target_entity);
            if let Some(target_transform) = target_transform_component {
                let player_position = &mut target_transform.position;
                let z_dest = match player_state_component {
                    Some(player_state_component) => player_state_component.cam_zoom(),
                    None => 0.0,
                };
                let x_max_dist = follow_component.max_distance;
                let y_max_dist = follow_component.max_distance;

                let treshold = 0.002;
                let mut difference =
                    Vec3::from_vec2(player_position.xy() - camera.position.xy(), 0.0);
                if difference.x.abs() < treshold {
                    difference.x = 0.0;
                }
                if difference.y.abs() < treshold {
                    difference.y = 0.0;
                }
                let direction = difference.normalized();
                let length = difference.length();

                if 0.0 < length {
                    if let Ok(updatables) = &mut updatables.lock() {
                        updatables.push_back(Updatable::View {
                            camera_component: camera.clone(),
                        });
                    }
                }

                camera.position +=
                    direction * camera.move_speed * f32::sqrt(length) * delta_time as f32;

                camera.position.x = camera.position.x.clamp(
                    player_position.x - x_max_dist,
                    player_position.x + x_max_dist,
                );

                camera.position.y = camera.position.y.clamp(
                    player_position.y - y_max_dist,
                    player_position.y + y_max_dist,
                );

                let z_curr = camera.position.z;
                let difference = z_dest - z_curr;
                let change = difference.signum() * (delta_time * camera.zoom_speed);

                if (z_dest - z_curr).abs() < change {
                    camera.position.z = z_dest;
                } else {
                    camera.position.z = z_curr + change;
                }
            }
        }
    }
}
