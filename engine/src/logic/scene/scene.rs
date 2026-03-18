use crate::{
    logic::{
        ecs::{
            component::camera::Camera,
            extract::{
                debug_extractor::DebugExtractor, extractor::Extractor,
                model_extractor::ModelExtractor, particle_extractor::ParticleExtractor,
                postprocess_extractor::PostprocessExtractor,
            },
            system::{
                collision_system::CollisionSystem, debug_system::DebugSystem,
                emitter_system::EmitterSystem, movement_system::MovementSystem,
                particle_system::ParticleSystem, system::System,
            },
        },
        scene::{
            focus_point::FocusPoint, matrix_uniform_buffer::MatrixUniformBuffer, world::World,
        },
    },
    math::vec3::Vec3,
    render::uniform_buffer_source::UniformBufferSource,
    shared::{
        extracted_frame::ExtractedFrame, input::input_state::InputState, window_size::WindowSize,
    },
};

pub struct Scene {
    pub systems: Vec<Box<dyn System>>,
    pub extractors: Vec<Box<dyn Extractor>>,
    world: World,
}

impl Scene {
    pub fn new() -> Self {
        let mut world = World::load();
        world.insert_resource(InputState::init());
        world.insert_resource(FocusPoint(Vec3::new(0.0, 0.0, 0.0)));
        world.insert_resource(WindowSize {
            width: 0,
            height: 0,
        });
        world.insert_resource(UniformBufferSource::new(
            0,
            MatrixUniformBuffer {
                projection_matrix: [[0.0; 4]; 4],
                view_matrix: [[0.0; 4]; 4],
            },
        ));
        let camera = world.create_entity();

        world.add_component(
            camera,
            Camera {
                position: Vec3::new(0.0, 0.25, 0.0),
                move_speed: 0.69,
                zoom_speed: 0.1,
                near: 0.0,
                far: 10.0,
            },
        );

        let systems: Vec<Box<dyn System>> = vec![
            Box::new(MovementSystem),
            Box::new(ParticleSystem),
            Box::new(EmitterSystem),
            Box::new(CollisionSystem),
            Box::new(DebugSystem),
        ];

        let extractors: Vec<Box<dyn Extractor>> = vec![
            Box::new(ModelExtractor),
            Box::new(ParticleExtractor),
            Box::new(DebugExtractor),
            Box::new(PostprocessExtractor),
        ];

        Scene {
            systems,
            world,
            extractors,
        }
    }

    pub fn register_system(&mut self, system: Box<dyn System>) {
        self.systems.insert(self.systems.len() - 1, system);
    }

    pub fn register_extractor(&mut self, extractor: Box<dyn Extractor>) {
        self.extractors.insert(self.extractors.len() - 1, extractor);
    }

    pub fn update(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.run(&mut self.world, delta_time);
        }
    }

    pub fn extract(&mut self) -> ExtractedFrame {
        let mut frame = ExtractedFrame {
            uniform_buffers: Vec::new(),
            entities: Vec::new(),
            window_size: None,
            postprocess_pass: None,
        };
        for extractor in &mut self.extractors {
            extractor.extract(&self.world, &mut frame);
        }
        frame
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
