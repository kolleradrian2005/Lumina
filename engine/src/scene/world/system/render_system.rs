use crate::{
    math::transformation,
    render::render_entity::RenderEntity,
    scene::world::{
        component::{
            emitter_component::EmitterComponent, material_component::MaterialComponent,
            model_component::ModelComponent, parent_component::ParentComponent,
            transform_component::TransformComponent,
        },
        entity::entity::Entity,
        world::World,
    },
};

use super::system::System;

pub struct RenderSystem;

impl System for RenderSystem {
    fn run(&self, world: &mut World, _: f32) {
        for (entity, (model, transform)) in
            world.query_mut::<(&mut ModelComponent, &mut TransformComponent)>()
        {
            let parent_component = world.get_component::<ParentComponent>(entity).cloned();
            if let None = world.get_component::<EmitterComponent>(entity) {
                Self::prepare_entity(world, entity, parent_component, transform.clone(), model);
            }
        }
        for (entity, (emitter, model, transform)) in world.query_mut::<(
            &mut EmitterComponent,
            &mut ModelComponent,
            &mut TransformComponent,
        )>() {
            for particle in emitter.particles.iter() {
                Self::prepare_entity(
                    world,
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

impl RenderSystem {
    pub fn prepare_entity(
        world: &mut World,
        entity: Entity,
        parent: Option<ParentComponent>,
        transform: TransformComponent,
        model: &mut ModelComponent,
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
        /*let create_mesh_manager = world.expect_resource::<Arc<Mutex<CreateMeshManager>>>();
        if let Ok(create_mesh_manager) = &mut create_mesh_manager.lock() {
            match model.mesh {
                MeshLoadState::CreateRequest { .. } => {
                    let create_mesh_request =
                        mem::replace(&mut model.mesh, MeshLoadState::PendingRequest { id: 0 });
                    if let MeshLoadState::PendingRequest { id } =
                        create_mesh_manager.request_mesh(create_mesh_request)
                    {
                        model.mesh = MeshLoadState::PendingRequest { id };
                    }
                    return;
                }
                MeshLoadState::PendingRequest { id } => {
                    if let Some(loaded_mesh) = create_mesh_manager.take_mesh(id) {
                        model.mesh = loaded_mesh;
                    }
                    return;
                }
                _ => {}
            }
        }*/
        world.render_packet.entities.push(RenderEntity {
            mesh: model.mesh.clone(),
            is_flipped: transform.is_flipped
                ^ parent_transform.map(|e| e.is_flipped).unwrap_or(false),
            //texture: texture.texture,
            transform_matrix,
            //object_type: model.object_type,
            //shader_params: world.get_component::<ShaderParamsComponent>(entity).cloned(),
            material: material,
        });
    }
}
