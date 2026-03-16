use super::system::System;
use crate::{
    logic::{
        ecs::{
            component::{emitter::Emitter, parent::Parent, transform::Transform},
            entity::entity::Entity,
        },
        scene::world::World,
    },
    math::{transformation::calc_intherited_transform, vec3::Vec3},
};
pub struct ParticleSystem;

impl System for ParticleSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        let transform_storage = world.get_storage_ptr_mut::<Transform>();
        let parent_storage = world.get_storage_ptr_mut::<Parent>();
        for (_, (emitter_component, parent_component)) in
            world.query_mut::<(&mut Emitter, &mut Parent)>()
        {
            if parent_component.parent == Entity(0).into() {
                continue;
            }
            let mut transform = Transform {
                position: Vec3::zero(),
                rotation: 0.0,
                scale: (1.0, 1.0).into(),
                is_flipped: false,
            };
            if let Some(transform_storage) =
                transform_storage.map(|transform_storage| unsafe { &mut *transform_storage })
            {
                if let Some(Some(parent_transform)) = transform_storage
                    .get(parent_component.parent)
                    .map(|e| e.downcast_ref::<Transform>())
                {
                    transform = calc_intherited_transform(&transform, parent_transform.into())
                }
                if let Some(parent_storage) =
                    parent_storage.map(|parent_storage| unsafe { &mut *parent_storage })
                {
                    if let Some(Some(parent_parent)) = parent_storage
                        .get(parent_component.parent)
                        .map(|e| e.downcast_ref::<Parent>())
                    {
                        if parent_parent.parent != Entity(0).into() {
                            if let Some(Some(parent_parent_transform)) = transform_storage
                                .get(parent_parent.parent)
                                .map(|e| e.downcast_ref::<Transform>())
                            {
                                transform = calc_intherited_transform(
                                    &transform,
                                    parent_parent_transform.into(),
                                );
                            }
                        }
                    }
                }
                emitter_component.spawn_position = transform.position;
            }
        }
    }
}
