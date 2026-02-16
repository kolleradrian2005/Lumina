
use lumina_engine::scene::world::{
        component::{camera_component::CameraComponent, transform_component::TransformComponent},
        system::system::System,
        world::World,
    };

use crate::components::follow_component::FollowComponent;

pub struct CameraSystem;

impl System for CameraSystem {
    fn run(&self, world: &mut World, _: f32) {
        /*let updatables = world
        .expect_resource::<Arc<Mutex<VecDeque<Updatable>>>>()
        .clone();*/
        for (_, (camera, follow_component)) in
            world.query_mut::<(&mut CameraComponent, &mut FollowComponent)>()
        {
            let target_transform_component = world
                .get_component_mut::<TransformComponent>(follow_component.target_entity)
                .unwrap();
            let old_focal_offset = camera.focal_offset.clone();
            camera.focal_offset = target_transform_component.position.xy() - camera.position.xy();
            if old_focal_offset != camera.focal_offset {
                /*if let Ok(updatables) = &mut updatables.lock() {
                    updatables.push_back(Updatable::View {
                        camera_component: camera.clone(),
                    });
                }*/
                world.render_packet.camera_component = Some(camera.clone());
            }
        }
    }
}
