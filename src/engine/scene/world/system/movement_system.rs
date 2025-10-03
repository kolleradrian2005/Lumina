

use crate::engine::{
    scene::{
        water::Water,
        world::{
            component::{
                movement_component::MovementComponent,
                transform_component::TransformComponent,
            },
            world::World,
        },
    },
};

use super::system::System;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&self, world: &mut World, delta_time: f32) {
        let water = world.expect_resource::<Water>();
        let water_resistance = water.get_resistance();
        for (_, (movement, transform)) in
            world.query_mut::<(&mut MovementComponent, &mut TransformComponent)>()
        {
            let direction_normal = movement.direction.normalized();
            movement.acceleration = direction_normal * movement.base_acceleration;
            movement.velocity += movement.acceleration * delta_time;
            movement.velocity *= f32::powf(1.0 - water_resistance, delta_time);

            let offset = movement.velocity * delta_time;
            transform.position += offset;
        }
    }
}
