use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;

use super::background::Background;
use super::world::component::camera_component::CameraComponent;
use super::world::system::collider_system::ColliderSystem;
use super::world::system::emitter_system::EmitterSystem;
use super::world::system::movement_system::MovementSystem;
use super::world::system::particle_system::ParticleSystem;
use super::world::system::render_system::RenderSystem;
use super::world::system::system::System;
use super::world::world::World;
use crate::scene::water::Water;
use crate::texture::resource_provider::ResourceProvider;

pub struct Scene {
    pub systems: Vec<Box<dyn System>>,
    world: World,
}

const WORLD_SEED: u32 = 696969;

impl Scene {
    pub fn new(resource_provider: &mut dyn ResourceProvider) -> Self {
        let mut world = World::load();
        let water = Water::create((WORLD_SEED ^ 0x5EAF00D).wrapping_mul(69696969));
        world.insert_resource(water);
        world.insert_resource(Background::construct(resource_provider));
        let camera = world.create_entity();

        world.add_component(
            camera,
            CameraComponent {
                position: Vec3::new(0.0, 0.25, 0.0),
                move_speed: 0.69,
                zoom_speed: 0.1,
                near: 0.0,
                far: 10.0,
                focal_offset: Vec2::new(0.0, 0.0),
            },
        );

        let systems: Vec<Box<dyn System>> = vec![
            Box::new(MovementSystem),
            Box::new(ParticleSystem),
            Box::new(EmitterSystem),
            Box::new(ColliderSystem),
            Box::new(RenderSystem),
        ];
        Scene { systems, world }
    }

    pub fn register_system(&mut self, system: Box<dyn System>) {
        self.systems.insert(self.systems.len() - 1, system);
    }

    pub fn update(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.run(&mut self.world, delta_time);
        }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
