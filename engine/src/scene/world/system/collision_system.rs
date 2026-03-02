use crate::{
    math::vec3::Vec3,
    scene::world::{
        collision_result::CollisionResult,
        component::{
            collider_component::ColliderComponent,
            force_component::{AppliedForce, ForceComponent, ForceEffect, ForceMode},
            transform_component::TransformComponent,
        },
        entity::entity::Entity,
        world::World,
    },
};

use super::system::System;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        let snapshot: Vec<(Entity, TransformComponent, ColliderComponent)> = world
            .query::<(&TransformComponent, &ColliderComponent)>()
            .map(|(entity, (transform, collider))| (entity, transform.clone(), collider.clone()))
            .collect();

        let mut collisions: Vec<CollisionResult> = vec![];
        for i in 0..snapshot.len() {
            for j in (i + 1)..snapshot.len() {
                let (entity_a, transform_a, collider_a) = &snapshot[i];
                let (entity_b, transform_b, collider_b) = &snapshot[j];
                if let Some((penetration_depth, normal)) = collider_a.intersect(
                    transform_a.position.xy().clone() + collider_a.offset,
                    transform_a.scale,
                    transform_a.rotation,
                    collider_b,
                    transform_b.position.xy().clone() + collider_b.offset,
                    transform_b.scale,
                    transform_b.rotation,
                ) {
                    collisions.push(CollisionResult {
                        entity_a: *entity_a,
                        entity_b: *entity_b,
                        collision_normal: normal,
                        penetration_depth: penetration_depth,
                    });
                }
            }
        }
        // TODO: isstatic?
        for collision in collisions {
            let push = collision.collision_normal * collision.penetration_depth * 40.0;
            if let Some(force) = world.get_component_mut::<ForceComponent>(collision.entity_a) {
                force.apply_force(AppliedForce {
                    id: format!("collision_{:?}", collision.entity_b),
                    effect: ForceEffect::Linear(Vec3::from_vec2(push, 0.0)),
                    mode: ForceMode::Impulse,
                });
            }
            if let Some(force) = world.get_component_mut::<ForceComponent>(collision.entity_b) {
                force.apply_force(AppliedForce {
                    id: format!("collision_{:?}", collision.entity_a),
                    effect: ForceEffect::Linear(Vec3::from_vec2(-push, 0.0)),
                    mode: ForceMode::Impulse,
                });
            }
        }
    }
}
