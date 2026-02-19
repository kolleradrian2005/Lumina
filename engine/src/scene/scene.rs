use winit::event::MouseButton;

use crate::focus_point::FocusPoint;
use crate::input::input_event::InputEvent;
use crate::input::input_state::InputState;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::scene::world::system::collision_system::CollisionSystem;
use crate::scene::world::system::emitter_system::EmitterSystem;

use super::world::component::camera_component::CameraComponent;
use super::world::system::movement_system::MovementSystem;
use super::world::system::particle_system::ParticleSystem;
use super::world::system::render_system::RenderSystem;
use super::world::system::system::System;
use super::world::world::World;

pub struct Scene {
    pub systems: Vec<Box<dyn System>>,
    world: World,
}

impl Scene {
    pub fn new() -> Self {
        let mut world = World::load();
        world.insert_resource(InputState::init());
        world.insert_resource(FocusPoint(Vec3::new(0.0, 0.0, 0.0)));
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

        // TODO: Prioritize systems
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(MovementSystem),
            Box::new(ParticleSystem),
            Box::new(EmitterSystem),
            Box::new(CollisionSystem),
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

    pub fn handle_input_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::WindowResize { width, height } => {
                let (_camera, (camera_component,)) = self
                    .world
                    .query_mut::<(&mut CameraComponent,)>()
                    .next()
                    .expect("No camera found in the scene");
                self.world.render_packet.camera_component = Some(camera_component.clone());
                self.world.render_packet.window_resize = Some((width, height));
            }
            InputEvent::KeyDown(key) => {
                self.world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .update_key_state(key, true);
            }
            InputEvent::KeyUp(key) => {
                self.world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .update_key_state(key, false);
            }
            InputEvent::MouseEvent { button, pressed } => {
                match button {
                    MouseButton::Left => {
                        self.world
                            .get_resource_mut::<InputState>()
                            .unwrap()
                            .set_l_mouse(pressed);
                    }
                    MouseButton::Right => {
                        self.world
                            .get_resource_mut::<InputState>()
                            .unwrap()
                            .set_l_mouse(pressed);
                    }
                    _ => {}
                };
            }
            InputEvent::MouseMove(vec2) => self
                .world
                .get_resource_mut::<InputState>()
                .unwrap()
                .update_mouse_position(vec2),
        };
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
