use lumina_engine::{
    focus_point::FocusPoint,
    render::{
        postprocess_config::PostprocessConfig,
        uniformbuffer::{MatrixUniformBuffer, UniformBufferSource},
        window_size::WindowSize,
    },
    scene::world::{
        component::{camera_component::CameraComponent, transform_component::TransformComponent},
        system::system::System,
        world::World,
    },
};

use crate::components::follow_component::FollowComponent;

pub struct CameraSystem;

impl System for CameraSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (camera, follow_component)) in
            world.query_mut::<(&mut CameraComponent, &mut FollowComponent)>()
        {
            let target_transform_component = world
                .get_component_mut::<TransformComponent>(follow_component.target_entity)
                .expect("Failed to get target transform component for camera follow component!");
            let focal_offset = target_transform_component.position.xy() - camera.position.xy();
            let window_size = world.expect_resource::<WindowSize>();
            let aspect = window_size.width as f32 / window_size.height as f32;
            if let Some(matrix_uniformbuffer) =
                world.get_resource_mut::<UniformBufferSource<MatrixUniformBuffer>>()
            {
                matrix_uniformbuffer.update(MatrixUniformBuffer {
                    projection_matrix: camera.get_projection_matrix(aspect),
                    view_matrix: camera.get_view_matrix(),
                });
            }
            if let Some(focus_point) = world.get_resource_mut::<FocusPoint>() {
                focus_point.0 = camera.position;
            }
            if let Some(postprocess_config) = world.get_resource_mut::<PostprocessConfig>() {
                postprocess_config
                    .material
                    .set_param("uAspectRatio", aspect);
                postprocess_config
                    .material
                    .set_param("uFocalOffset", focal_offset);
            }
        }
    }
}
