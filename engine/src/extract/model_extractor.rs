use crate::{
    extract::extractor::Extractor,
    math::transformation,
    render::{
        extracted_frame::ExtractedFrame, render_entity::RenderEntity, window_size::WindowSize,
    },
    scene::world::{
        component::{
            camera_component::CameraComponent, emitter_component::EmitterComponent,
            material_component::MaterialComponent, model_component::ModelComponent,
            parent_component::ParentComponent, transform_component::TransformComponent,
        },
        entity::entity::Entity,
        world::World,
    },
};

pub struct ModelExtractor;

impl Extractor for ModelExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        let (_camera, (camera_component,)) = world
            .query::<(&CameraComponent,)>()
            .next()
            .expect("No camera found in the scene");
        frame.camera_component = Some(camera_component.clone());
        let window_size = world.get_resource::<WindowSize>();
        frame.window_size = window_size.cloned();
        for (entity, (model, transform)) in world.query::<(&ModelComponent, &TransformComponent)>()
        {
            let parent_component = world.get_component::<ParentComponent>(entity).cloned();
            if let None = world.get_component::<EmitterComponent>(entity) {
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
        for (entity, (emitter, model, transform)) in
            world.query::<(&EmitterComponent, &ModelComponent, &TransformComponent)>()
        {
            for particle in emitter.particles.iter() {
                Self::prepare_entity(
                    world,
                    frame,
                    entity,
                    None,
                    TransformComponent {
                        position: transform.position + particle.position,
                        rotation: transform.rotation,
                        scale: transform.scale,
                        is_flipped: transform.is_flipped,
                    },
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
        parent: Option<ParentComponent>,
        transform: TransformComponent,
        model: &ModelComponent,
    ) {
        if let Some(parent) = parent.clone() {
            if parent.parent.0 == 0 {
                return;
            }
        }
        let parent_transform = parent
            .map(|parent| world.get_component::<TransformComponent>(parent.parent))
            .unwrap_or(None);
        let transform_matrix =
            transformation::create_transform_matrix(&transform, parent_transform);
        let material = world.get_component::<MaterialComponent>(entity).cloned();
        if material.is_none() {
            return;
        }
        let material = material.unwrap();
        // TODO: default material
        //.unwrap_or(MaterialComponent::default());
        frame.entities.push(RenderEntity {
            mesh: model.mesh.clone(),
            is_flipped: transform.is_flipped
                ^ parent_transform.map(|e| e.is_flipped).unwrap_or(false),
            //texture: texture.texture,
            transform_matrix,
            //object_type: model.object_type,
            //shader_params: world.get_component::<ShaderParamsComponent>(entity).cloned(),
            material: material,
            z_index: transform.position.z + parent_transform.map(|e| e.position.z).unwrap_or(0.0),
        });
    }
}
