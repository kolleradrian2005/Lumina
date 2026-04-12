use crate::{
    logic::{
        ecs::{
            component::{
                emitter::Emitter, material::Material, model::Model, parent::Parent,
                transform::Transform,
            },
            entity::entity::Entity,
            extract::extractor::Extractor,
        },
        scene::{matrix_uniform_buffer::MatrixUniformBuffer, world::World},
    },
    math::transformation::{self, get_world_transform},
    render::uniform_buffer_source::UniformBufferSource,
    shared::{
        extracted_frame::ExtractedFrame, render_entity::RenderEntity, window_size::WindowSize,
    },
};
pub struct ModelExtractor;

impl Extractor for ModelExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        if let Some(source) = world.get_resource::<UniformBufferSource<MatrixUniformBuffer>>() {
            frame.uniform_buffers.push(source.extract());
        }
        let window_size = world.get_resource::<WindowSize>();
        frame.window_size = window_size.cloned();
        for (entity, (model, transform)) in world.query::<(&Model, &Transform)>() {
            let parent_component = world.get_component::<Parent>(entity).cloned();
            if let None = world.get_component::<Emitter>(entity) {
                Self::prepare_entity(
                    world,
                    frame,
                    entity,
                    parent_component,
                    transform.clone(),
                    model,
                );
            }
        }
    }
}

impl ModelExtractor {
    pub fn prepare_entity(
        world: &World,
        frame: &mut ExtractedFrame,
        entity: Entity,
        parent: Option<Parent>,
        transform: Transform,
        model: &Model,
    ) {
        if let Some(parent) = &parent {
            if parent.parent.0 == 0 {
                return;
            }
        }
        let parent_world_transform = parent.as_ref().and_then(|parent| {
            get_world_transform(
                parent.parent,
                &|e| world.get_component::<Transform>(e).cloned(),
                &|e| world.get_component::<Parent>(e).cloned(),
            )
        });
        let transform_matrix =
            transformation::create_transform_matrix(&transform, parent_world_transform.as_ref());
        let material = world.get_component::<Material>(entity).cloned();
        if material.is_none() {
            return;
        }
        let mut material = material.unwrap();
        let is_flipped = transform.is_flipped
            ^ parent_world_transform
                .as_ref()
                .map(|e| e.is_flipped)
                .unwrap_or(false);
        material.set_param("uModelMatrix", transform_matrix);
        material.set_param("uFlipped", is_flipped as i32);
        frame.entities.push(RenderEntity {
            mesh: model.mesh.clone(),
            material: material,
            z_index: transform.position.z
                + parent_world_transform
                    .as_ref()
                    .map(|e| e.position.z)
                    .unwrap_or(0.0),
        });
    }
}
