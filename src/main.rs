extern crate gl;

#[path = "engine/window_handler.rs"] mod window_handler;
#[path = "engine/input_handler.rs"] mod input_handler;
#[path = "engine/mesh_handler.rs"] mod mesh_handler;
#[path = "engine/shader.rs"] mod shader;
#[path = "engine/shader_program.rs"] mod shader_program;
#[path = "engine/renderer.rs"] mod renderer;
#[path = "engine/scene.rs"] mod scene;
#[path = "engine/scene_renderer.rs"] mod scene_renderer;
#[path = "engine/model.rs"] mod model;
#[path = "engine/vec2.rs"] mod vec2;
#[path = "engine/texture.rs"] mod texture;

use crate::renderer::Renderer;
use crate::window_handler::WindowHandler;
use crate::scene::Scene;
use crate::model::Model;

fn main() {
    // Create window & renderer
    let mut window_handler: WindowHandler = WindowHandler::new();
    let renderer: Renderer = Renderer::init(&mut window_handler.window);
    // Set up scene
    //let mesh_handler: MeshHandler = MeshHandler::new();
    let mut scene = Scene::new();
    // TODO: example model
    let vertices: &[f32] = &[
        -0.5, -0.5,
        0.5, -0.5,
        0.0, 0.5,
    ];
    let indices: &[u32] = &[
        0, 1, 2
    ];
    let model = Model::new(vertices, indices);
    scene.add_model(model);
    while !window_handler.should_close() {
        // Handle user input
        window_handler.handle_events();
        // Clear screen
        renderer.clean_up();
        // Render stuff
        renderer.render(&scene);
        // Draw the buffer
        window_handler.swap_buffers();
    }
}
