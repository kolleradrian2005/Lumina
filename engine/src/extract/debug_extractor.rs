use crate::{
    extract::extractor::Extractor,
    math::{transformation, vec3::Vec3},
    render::{extracted_frame::ExtractedFrame, render_entity::RenderEntity},
    scene::world::{
        component::{
            collider_component::ColliderComponent,
            material_component::{DrawMode, MaterialComponent},
            transform_component::TransformComponent,
        },
        world::World,
    },
    texture::{
        resource_manager::{ColliderShapeKey, ResourceManager},
        resource_provider::ResourceProvider,
        texture::{StaticColor, Texture},
    },
};

pub struct DebugExtractor;

impl Extractor for DebugExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        for (_, (collider, transform)) in world.query::<(&ColliderComponent, &TransformComponent)>()
        {
            if let Some(resource_manager) = world.get_resource::<ResourceManager>() {
                let key = ColliderShapeKey::from_shape(&collider.shape);
                if let Some(mesh) = resource_manager.get_collider_mesh(key) {
                    let debug_shader = resource_manager.get_shader("debug_shader");
                    let mut material = MaterialComponent::new(
                        Texture::StaticColor(StaticColor::new((1.0, 0.0, 0.0).into())),
                        debug_shader,
                    )
                    .with_draw_mode(DrawMode::Lines);
                    let mut transform = transform.clone();
                    transform.position += Vec3::from_vec2(collider.offset, 0.0);
                    material.set_param(
                        "uModelMatrix",
                        transformation::create_transform_matrix(&transform, None), // TODO: maybe check for parent components in the future
                    );
                    frame.entities.push(RenderEntity {
                        mesh: mesh.clone(),
                        material,
                        z_index: transform.position.z,
                    });
                }
            }
        }
    }
}
