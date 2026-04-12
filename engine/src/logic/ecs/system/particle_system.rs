use super::system::System;
use crate::{
    logic::{
        ecs::{
            component::{emitter::Emitter, parent::Parent, transform::Transform},
            entity::entity::Entity,
        },
        scene::world::World,
    },
    math::{transformation::get_world_transform, vec3::Vec3},
};
pub struct ParticleSystem;

impl System for ParticleSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        let transform_storage = world.get_storage_ptr_mut::<Transform>();
        for (entity, (emitter_component, parent_component)) in
            world.query_mut::<(&mut Emitter, &mut Parent)>()
        {
            if parent_component.parent == Entity(0).into() {
                continue;
            }
            let mut emitter_offset = Vec3::zero();
            if let Some(transform_storage) =
                transform_storage.map(|transform_storage| unsafe { &mut *transform_storage })
            {
                if let Some(Some(own_transform)) = transform_storage
                    .get(entity)
                    .map(|e| e.downcast_ref::<Transform>())
                {
                    emitter_offset = own_transform.position;
                }
            }

            let parent_world = get_world_transform(
                parent_component.parent,
                &|e| world.get_component::<Transform>(e).cloned(),
                &|e| world.get_component::<Parent>(e).cloned(),
            );

            if let Some(parent_world) = parent_world {
                let rot = parent_world.rotation;
                let is_flipped = parent_world.is_flipped;
                let flip_mul = if is_flipped { -1.0f32 } else { 1.0f32 };
                let rotated_offset = emitter_offset.xy().rotated(rot * flip_mul);
                let final_offset = Vec3::new(
                    rotated_offset.x * flip_mul,
                    rotated_offset.y,
                    emitter_offset.z,
                );
                emitter_component.spawn_position = parent_world.position + final_offset;
            }
        }
    }
}
