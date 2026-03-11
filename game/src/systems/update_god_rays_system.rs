use lumina_engine::render::postprocess_config::PostprocessConfig;
use lumina_engine::scene::world::system::system::System;
use lumina_engine::{
    math::vec3::Vec3,
    scene::{
        foreground::Foreground,
        world::{component::camera_component::CameraComponent, world::World},
    },
};

pub struct UpdateGodRaysSystem;

impl System for UpdateGodRaysSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        let camera_position: Option<Vec3> = world
            .query_mut::<(&mut CameraComponent,)>()
            .last()
            .map(|(_, (camera,))| camera.position.clone());

        if let Some(camera_position) = camera_position {
            if let Some(foreground) = world.get_resource_ptr::<Foreground>() {
                let foreground = unsafe { &mut *foreground };
                let noise_index = (camera_position.x / foreground.god_rays_min_distance) as i32;
                let difference = foreground.loaded_noise_index - noise_index;
                if difference != 0 {
                    foreground.loaded_noise_index = noise_index;
                    if 0 < difference {
                        let x = foreground.loaded_noise_index - foreground.god_rays_max_count / 2;
                        let noise_value = foreground.get_noise_value(x);
                        foreground.god_rays_noise.push_front(noise_value);
                        foreground.god_rays_noise.pop_back();
                    } else {
                        let x = foreground.loaded_noise_index + foreground.god_rays_max_count / 2;
                        let noise_value = foreground.get_noise_value(x);
                        foreground.god_rays_noise.push_back(noise_value);
                        foreground.god_rays_noise.pop_front();
                    }
                }
                if let Some(postprocess_config) = world.get_resource_mut::<PostprocessConfig>() {
                    let light_positions = foreground.get_light_positions();
                    postprocess_config
                        .material
                        .set_param("uNumLights", light_positions.len() as i32);
                    postprocess_config
                        .material
                        .set_param("uLightPositions", light_positions);
                }
            }
        }
    }
}
