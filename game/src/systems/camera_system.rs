use lumina_engine::{
    focus_point::FocusPoint,
    scene::world::{
        component::{camera_component::CameraComponent, transform_component::TransformComponent},
        system::system::System,
        world::World,
    },
};

use crate::components::follow_component::FollowComponent;

pub struct CameraSystem;

impl System for CameraSystem {
    fn run(&self, world: &mut World, _: f32) {
        for (_, (camera, follow_component)) in
            world.query_mut::<(&mut CameraComponent, &mut FollowComponent)>()
        {
            let target_transform_component = world
                .get_component_mut::<TransformComponent>(follow_component.target_entity)
                .expect("Failed to get target transform component for camera follow component!");
            let old_focal_offset = camera.focal_offset.clone();
            camera.focal_offset = target_transform_component.position.xy() - camera.position.xy();
            if old_focal_offset != camera.focal_offset {
                world.render_packet.camera_component = Some(camera.clone());
            }
            if let Some(focus_point) = world.get_resource_mut::<FocusPoint>() {
                focus_point.0 = camera.position;
            }
        }
    }
}
