use crate::{
    math::vec3::Vec3,
    scene::world::{
        component::{
            force_component::ForceComponent, movement_component::MovementComponent,
            transform_component::TransformComponent,
        },
        world::World,
    },
};

use super::system::System;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&self, world: &mut World, delta_time: f32) {
        for (_, (movement, force, transform)) in world.query_mut::<(
            &mut MovementComponent,
            &mut ForceComponent,
            &mut TransformComponent,
        )>() {
            let net_force: Vec3 = force
                .get_linear_force_vecs()
                .fold(Vec3::zero(), |acc, f| acc + f);
            movement.acceleration = net_force * (1.0 / force.mass);
            movement.velocity += movement.acceleration * delta_time;
            for drag_force_factor in force.get_drag_force_factors() {
                movement.velocity *= f32::powf(1.0 - drag_force_factor, delta_time);
            }
            transform.position += movement.velocity * delta_time;
            force.clear_impulses();
        }

        /*let water = world.expect_resource::<Water>();
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
        }*/
    }
}
