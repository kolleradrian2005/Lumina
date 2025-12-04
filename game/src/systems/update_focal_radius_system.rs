use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use lumina_engine::scene::world::system::system::System;

use lumina_engine::{
    math::vec3::Vec3,
    render::updatable::Updatable,
    scene::{
        foreground::Foreground,
        world::{component::transform_component::TransformComponent, world::World},
    },
};

use crate::components::player_state_component::PlayerStateComponent;

pub struct UpdateFocalRadiusSystem;

impl System for UpdateFocalRadiusSystem {
    fn run(&self, world: &mut World, delta_time: f32) {
        let updatables = world
            .expect_resource::<Arc<Mutex<VecDeque<Updatable>>>>()
            .clone();
        let mut player_position: Option<Vec3> = None;
        let mut light_level: Option<f32> = None;

        for (_, (player_state_component, transform)) in
            world.query_mut::<(&mut PlayerStateComponent, &mut TransformComponent)>()
        {
            player_position = transform.position.clone().into();
            light_level = player_state_component.light_level().into();
        }
        if let Some(_player_position) = player_position {
            if let Some(foreground) = world.get_resource::<Arc<Mutex<Foreground>>>() {
                if let Ok(foreground) = &mut foreground.lock() {
                    let focal_dest = light_level.unwrap_or(0.5);
                    let difference = focal_dest - foreground.focal_radius;
                    if 0.0 < difference.abs() {
                        if let Ok(updatables) = &mut updatables.lock() {
                            updatables.push_front(Updatable::FocalRadius);
                        }
                    }
                    let change = difference.signum() * (delta_time * foreground.focus_speed);
                    if (focal_dest - foreground.focal_radius).abs() < change {
                        foreground.focal_radius = focal_dest;
                    } else {
                        foreground.focal_radius += change;
                    }
                }
            }
        }
    }
}
