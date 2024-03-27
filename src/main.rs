extern crate gl;

pub mod engine {
    pub mod gui {
        pub mod elements {
            pub mod align;
            pub mod column;
            pub mod container;
            pub mod gesture_detector;
            pub mod padding;
            pub mod row;
            pub mod text;
        }
        pub mod game_gui;
        pub mod gui;
        pub mod gui_element;
        pub mod gui_manager;
        pub mod listener;
        pub mod ui_model;
        pub mod ui_model_group;
    }
    pub mod math {
        pub mod rect;
        pub mod transformation;
        pub mod vec2;
        pub mod vec3;
    }
    pub mod model {
        pub mod mesh_handler;
        pub mod model;
        pub mod model_group;
        pub mod sprite;
    }
    pub mod render {
        pub mod background_renderer;
        pub mod gui_renderer;
        pub mod postprocess_renderer;
        pub mod renderer;
        pub mod scene_renderer;
        pub mod uniformbuffer;
        pub mod updatable;
    }
    pub mod scene {
        pub mod particle {
            pub mod bubble;
            pub mod fish;
            pub mod particle;
            pub mod particle_system;
        }
        pub mod background;
        pub mod camera;
        pub mod foreground;
        pub mod player;
        pub mod scene;
        pub mod terrain;
        pub mod tile;
        pub mod water;
        pub mod world;
    }
    pub mod shader {
        pub mod background_shader;
        pub mod gui_shader;
        pub mod model_shader;
        pub mod postprocess_shader;
        pub mod shader;
        pub mod shader_handler;
        pub mod shader_program;
        pub mod terrain_shader;
    }
    pub mod texture {
        pub mod font_texture;
        pub mod frame_buffer;
        pub mod resource_manager;
        pub mod texture;
        pub mod texture_handler;
    }
    pub mod collider;
    pub mod input_handler;
    pub mod references;
    pub mod transformable;
    pub mod window_handler;
}

use engine::gui::gui_manager::GuiManager;
use engine::render::renderer::Renderer;
use engine::render::updatable::Updatable;
use engine::scene::scene::Scene;
use engine::texture::resource_manager::ResourceManager;
use engine::transformable;
use engine::window_handler::WindowHandler;
use rand::rngs::ThreadRng;
use std::time::{Duration, Instant};

fn main() {
    // Create main controllers
    let mut window_handler: WindowHandler = WindowHandler::new();
    let mut renderer: Renderer = Renderer::init(&mut window_handler);
    let mut resource_manager: ResourceManager = ResourceManager::new();
    resource_manager.preload_models();
    resource_manager.load_fonts();
    let mut gui_manager: GuiManager = GuiManager::new();
    gui_manager.build(&resource_manager, &window_handler);
    let mut rng: ThreadRng = rand::thread_rng();
    let mut scene: Scene = Scene::new(&mut resource_manager);
    renderer.load_scene(&scene, &window_handler);
    let mut delta_time: Duration;
    let mut now: Instant;
    let mut last: Instant = Instant::now();
    let mut updatables: Vec<Updatable> = Vec::new();
    // Main loop
    while !window_handler.should_close() {
        // Calculate delta time
        now = Instant::now();
        delta_time = now.duration_since(last);
        last = now;
        // Handle user input
        window_handler.handle_events(&mut updatables);
        // Handle ui logic
        gui_manager.update(&resource_manager, &mut window_handler);
        // Update the scene
        scene.update(
            delta_time.as_secs_f32(),
            window_handler.get_input_handler(),
            &mut resource_manager,
            &mut rng,
            &mut updatables,
        );
        // Update buffers based on scene
        renderer.update_buffers(&mut updatables, &mut scene, &window_handler);
        // Render stuff
        renderer.render(&scene, &gui_manager, &window_handler);
        // Draw the buffer
        window_handler.swap_buffers();
    }
}
