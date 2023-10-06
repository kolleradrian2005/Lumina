extern crate gl;

#[path = "engine/references.rs"] mod references;
#[path = "engine/window_handler.rs"] mod window_handler;
#[path = "engine/input_handler.rs"] mod input_handler;
#[path = "engine/mesh_handler.rs"] mod mesh_handler;
#[path = "engine/texture_handler.rs"] mod texture_handler;
#[path = "engine/model.rs"] mod model;
#[path = "engine/texture.rs"] mod texture;
#[path = "engine/physics.rs"] mod physics;
#[path = "engine/frame_buffer.rs"] mod frame_buffer;
#[path = "engine/scene/scene.rs"] mod scene;
#[path = "engine/scene/background.rs"] mod background;
#[path = "engine/scene/foreground.rs"] mod foreground;
#[path = "engine/scene/player.rs"] mod player;
#[path = "engine/scene/camera.rs"] mod camera;
#[path = "engine/scene/terrain.rs"] mod terrain;
#[path = "engine/math/vec2.rs"] mod vec2;
#[path = "engine/math/vec3.rs"] mod vec3;
#[path = "engine/math/transformation.rs"] mod transformation;
#[path = "engine/render/renderer.rs"] mod renderer;
#[path = "engine/render/scene_renderer.rs"] mod scene_renderer;
#[path = "engine/render/player_renderer.rs"] mod player_renderer;
#[path = "engine/render/terrain_renderer.rs"] mod terrain_renderer;
#[path = "engine/render/background_renderer.rs"] mod background_renderer;
#[path = "engine/render/postprocess_renderer.rs"] mod postprocess_renderer;
#[path = "engine/shader/shader.rs"] mod shader;
#[path = "engine/shader/shader_program.rs"] mod shader_program;

use std::time::{SystemTime, UNIX_EPOCH};

use physics::Physics;
use texture_handler::TextureHandler;

use crate::renderer::Renderer;
use crate::window_handler::WindowHandler;
use crate::scene::Scene;
use crate::model::Model;

fn main() {
    // Create window & renderer
    let mut window_handler: WindowHandler = WindowHandler::new();
    let mut renderer: Renderer = Renderer::init(&mut window_handler.window);
    //let mesh_handler: MeshHandler = MeshHandler::new();
    let mut texture_handler = TextureHandler::new();
    // Set up scene
    let mut scene = Scene::new(&mut texture_handler);
    // Create example model
    let vertices: &[f32] = &[
        -0.25, -0.25, 1.0,
        0.25, -0.25, 1.0,
        0.25, 0.25, 1.0,
        -0.25, 0.25, 1.0
    ];
    let indices: &[u32] = &[
        0, 1, 2,
        2, 3, 0
    ];
    let uvs: &[f32] = &[
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
    ];
    let mut model = Model::new(vertices, indices, uvs);
    model.load_single_texture(&mut texture_handler, "texture.png");
    scene.add_model(model);
    
    let physics = Physics::new();
    let mut delta_time: u128;
    let mut now: u128;
    let mut last = get_time();
    while !window_handler.should_close() {
        now = get_time();
        delta_time = now - last;
        last = now;
        // Handle user input
        window_handler.handle_events(&mut scene, delta_time);
        // Do physics
        physics.update(&mut scene, delta_time);
        // Clear screen
        renderer.clean_up();
        // Render stuff
        renderer.render(&mut scene, &mut texture_handler);
        // Draw the buffer
        window_handler.swap_buffers();
    }
}

fn get_time() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Error");
    since_the_epoch.as_millis()
}
