use crate::{
    math::transformation,
    render::render_entity::RenderEntity,
    scene::world::{
        component::{
            emitter_component::EmitterComponent, material_component::MaterialComponent,
            model_component::ModelComponent, parent_component::ParentComponent,
            shader_params_component::ShaderParamsComponent, texture_component::TextureComponent,
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
        for (entity, (model, transform)) in
            world.query_mut::<(&mut ModelComponent, &mut TransformComponent)>()
        {
            let parent_component = world.get_component::<ParentComponent>(entity).cloned();
            Self::prepare_entity(world, entity, parent_component, transform.clone(), model);
        }
    }
}

impl RenderSystem {
    pub fn prepare_entity(
        world: &mut World,
        entity: Entity,
        parent: Option<ParentComponent>,
        transform: TransformComponent,
        material_component: MaterialComponent,
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
        let texture = world
            .get_component::<TextureComponent>(entity)
            .cloned()
            .unwrap_or(TextureComponent::default());
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
            material: material_component,
        });
    }
}
