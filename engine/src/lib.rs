extern crate gl;
extern crate glutin;
extern crate glutin_winit;

pub mod render_context;
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
    pub mod mesh;
    pub mod model;
    pub mod sprite;
}
pub mod render {
    pub mod background_renderer;
    pub mod gui_renderer;
    pub mod postprocess_renderer;
    pub mod renderable;
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
        //pub mod particle_system;
    }
    pub mod world {
        pub mod entity {
            pub mod entity;
            pub mod particle_entity;
        }
        pub mod component {
            pub mod camera_component;
            pub mod collider_component;
            pub mod component;
            pub mod conditional_parent_component;
            pub mod current_component;
            pub mod emitter_component;
            pub mod model_component;
            pub mod movement_component;
            pub mod multi_conditional_parent_component;
            pub mod parent_component;
            pub mod player_part_component;
            pub mod player_state_component;
            pub mod shader_params_component;
            pub mod texture_component;
            pub mod transform_component;
        }
        pub mod system {
            pub mod animation_system;
            pub mod camera_system;
            pub mod collider_system;
            pub mod current_system;
            pub mod emitter_system;
            pub mod input_system;
            pub mod movement_system;
            pub mod particle_system;
            pub mod player_movement_system;
            pub mod render_system;
            pub mod system;
            pub mod terrain_system;
            pub mod update_focal_radius_system;
            pub mod update_god_rays_system;
        }
        pub mod component_storage;
        pub mod create_mesh_manager;
        pub mod drop_mesh_request;
        pub mod query;
        pub mod world;
    }
    pub mod background;
    pub mod foreground;
    pub mod player_state;
    pub mod scene;
    pub mod terrain;
    pub mod tile;
    pub mod water;
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
    pub mod resource_provider;
    pub mod texture;
    pub mod texture_handler;
}
pub mod collider;
pub mod input_handler;
pub mod references;
pub mod transformable;
pub mod window_handler;

use input_handler::InputHandler;
use math::vec2::Vec2;
use render::updatable::Updatable;

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use rand::{rngs::StdRng, SeedableRng};
use raw_window_handle::HasRawWindowHandle;
use render_context::RenderContext;
use std::borrow::BorrowMut;
use std::collections::VecDeque;

use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, ElementState, Event, KeyEvent, TouchPhase, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowBuilder};

use crate::scene::world::create_mesh_manager::CreateMeshManager;
use crate::scene::world::drop_mesh_request::DropMeshRequest;

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn build_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_transparent(true)
        .with_title(references::WINDOW_TITLE)
        .with_inner_size(PhysicalSize::new(
            references::INITIAL_WINDOW_WIDTH,
            references::INITIAL_WINDOW_HEIGHT,
        ))
}

#[tokio::main]
pub async fn start(event_loop: EventLoop<()>) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(3)
        .enable_all()
        .build()
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let window_builder = cfg!(not(target_os = "android")).then(build_window);

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(false);

    let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

    let (mut window, gl_config) = display_builder
        .build(&event_loop, template, gl_config_picker)
        .unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let mut not_current_gl_context: Option<NotCurrentContext> = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("Failed to create context!")
                    })
            })
    });

    let mut state: Option<(PossiblyCurrentContext, Surface<WindowSurface>, Window)> = None;
    let updatables: Arc<Mutex<VecDeque<Updatable>>> = Arc::new(Mutex::new(VecDeque::new()));
    let drop_mesh_requests: Arc<Mutex<VecDeque<DropMeshRequest>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let create_mesh_manager: Arc<Mutex<CreateMeshManager>> =
        Arc::new(Mutex::new(CreateMeshManager::new()));

    let is_running = Arc::new(Mutex::new(true));

    // Two instances of the same input handler for more efficent concurrency
    let mut input_handler = InputHandler::init();
    let loop_input_handler = Arc::new(Mutex::new(InputHandler::init()));
    let loop_input_handler_clone = Arc::clone(&loop_input_handler);

    // Two instances of the same render context for more efficent concurrency
    let mut render_context: Option<RenderContext> = None;
    let loop_render_context: Arc<Mutex<Option<RenderContext>>> = Arc::new(Mutex::new(None));
    let loop_render_context_clone: Arc<Mutex<Option<RenderContext>>> =
        Arc::clone(&loop_render_context);

    let updatables_clone: Arc<Mutex<VecDeque<Updatable>>> = Arc::clone(&updatables);
    let drop_mesh_requests_clone: Arc<Mutex<VecDeque<DropMeshRequest>>> =
        Arc::clone(&drop_mesh_requests);
    let create_mesh_manager_clone: Arc<Mutex<CreateMeshManager>> = Arc::clone(&create_mesh_manager);
    let is_running_clone = Arc::clone(&is_running);

    const TARGET_INTERVAL: Duration = Duration::from_millis(10);

    let main_loop = tokio::spawn(async move {
        tokio::time::sleep(TARGET_INTERVAL).await;
        let rng: StdRng = SeedableRng::from_entropy();
        let mut delta_time: Duration;
        let mut last: Instant = Instant::now();

        loop {
            if let Some(render_ctx) = loop_render_context_clone.lock().unwrap().as_ref() {
                if let Ok(scene) = &mut render_ctx.scene.lock() {
                    let world = scene.get_world_mut();
                    world.insert_resource(render_ctx.resource_handle.get_inner());
                    world.insert_resource(loop_input_handler_clone.clone());
                    world.insert_resource(updatables_clone.clone());
                    world.insert_resource(drop_mesh_requests_clone.clone());
                    world.insert_resource(create_mesh_manager_clone.clone());
                    world.insert_resource(rng);
                }
                break;
            }
        }

        while *is_running_clone.lock().unwrap() {
            // Calculate delta time
            delta_time = last.elapsed();
            if delta_time < TARGET_INTERVAL {
                time::sleep(TARGET_INTERVAL - delta_time).await;
            }
            delta_time = Duration::max(delta_time, TARGET_INTERVAL);
            last = Instant::now();

            if let Some(render_ctx) = loop_render_context_clone.lock().unwrap().as_ref() {
                // Update the scene
                if let Ok(scene) = &mut render_ctx.scene.lock() {
                    scene.update(delta_time.as_secs_f32());
                }
            }
        }
    });
    event_loop
        .run(move |event, window_target| {
            log::debug!("Incoming event: {:?}", event);
            match event {
                Event::Resumed => {
                    let window = window.take().unwrap_or_else(|| {
                        let window_builder = build_window();
                        glutin_winit::finalize_window(window_target, window_builder, &gl_config)
                            .unwrap()
                    });
                    let attrs = window.build_surface_attributes(Default::default());
                    let gl_surface = unsafe {
                        gl_config
                            .display()
                            .create_window_surface(&gl_config, &attrs)
                            .unwrap()
                    };

                    let gl_context = not_current_gl_context
                        .take()
                        .unwrap()
                        .make_current(&gl_surface)
                        .unwrap();

                    let width = gl_surface.width().unwrap() as i32;
                    let height = gl_surface.height().unwrap() as i32;
                    if let Err(res) = gl_surface.set_swap_interval(
                        &gl_context,
                        SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                    ) {
                        println!("Error setting vsync: {res:?}");
                    }
                    let render_ctx = RenderContext::init(&gl_display, width, height);
                    render_context.get_or_insert_with(|| render_ctx.clone());
                    loop_render_context
                        .lock()
                        .unwrap()
                        .get_or_insert_with(|| render_ctx);
                    state.replace((gl_context, gl_surface, window));
                }
                Event::Suspended => {
                    let (gl_context, ..) = state.take().unwrap();
                    not_current_gl_context.replace(gl_context.make_not_current().unwrap());
                }
                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => match event {
                    DeviceEvent::Button { button, state } => match button {
                        0 => loop_input_handler
                            .lock()
                            .unwrap()
                            .set_l_mouse(state.is_pressed()),
                        1 => loop_input_handler
                            .lock()
                            .unwrap()
                            .set_r_mouse(state.is_pressed()),
                        _ => {}
                    },
                    _ => {}
                },
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::RedrawRequested => {
                        if let Some(render_ctx) = &render_context {
                            // Handle ui logic
                            if let Ok(gui_manager) = &mut render_ctx.gui_manager.lock() {
                                if let Ok(resource_provider) = render_ctx.resource_handle.lock() {
                                    gui_manager.update(
                                        &*resource_provider,
                                        loop_input_handler.lock().unwrap().borrow_mut(),
                                    );
                                }
                            }
                            if let Ok(renderer) = &mut render_ctx.renderer.lock() {
                                if let Ok(scene) = &mut render_ctx.scene.lock() {
                                    // Update buffers based on scene
                                    if let Ok(updatables) = &mut updatables.lock() {
                                        renderer.update_buffers(updatables, scene);
                                    }
                                    if let Ok(gui_manager) = &mut render_ctx.gui_manager.lock() {
                                        let drop_mesh_requests =
                                            &mut drop_mesh_requests.lock().unwrap();
                                        let create_mesh_manager =
                                            &mut create_mesh_manager.lock().unwrap();
                                        renderer.handle_mesh_requests(
                                            drop_mesh_requests,
                                            create_mesh_manager,
                                        );
                                        // Render stuff
                                        renderer.render(scene, gui_manager);
                                    }
                                }
                            }
                            if let Some((_, _, window)) = &state {
                                window.pre_present_notify();
                            }
                            if let Some((gl_context, gl_surface, _)) = &state {
                                gl_surface
                                    .swap_buffers(gl_context)
                                    .expect("Error swapping buffers!");
                            }
                        }
                        if let Some((_, _, window)) = &state {
                            window.request_redraw();
                        }
                    }
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            if let Some((gl_context, gl_surface, _)) = &state {
                                gl_surface.resize(
                                    gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );
                                let width = size.width as i32;
                                let height = size.height as i32;
                                updatables
                                    .lock()
                                    .unwrap()
                                    .push_back(Updatable::Projection { width, height });
                                if let Some(render_ctx) = &render_context {
                                    if let Ok(gui_manager) = &mut render_ctx.gui_manager.lock() {
                                        gui_manager.resize((width, height));
                                        if let Ok(resource_provider) =
                                            &render_ctx.resource_handle.lock()
                                        {
                                            gui_manager.build(
                                                &**resource_provider,
                                                width as f32 / height as f32,
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    } => {
                        window_target.exit();
                    }
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event:
                            KeyEvent {
                                logical_key, state, ..
                            },
                        is_synthetic: _,
                    } => {
                        let new_state = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };
                        if input_handler.update_key_state(logical_key.clone(), new_state) {
                            let _ = loop_input_handler
                                .lock()
                                .unwrap()
                                .update_key_state(logical_key, new_state);
                        }
                    }
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position: pos,
                    } => handle_cursor_movement(&loop_input_handler, pos),
                    WindowEvent::Touch(touch) => match touch.phase {
                        TouchPhase::Moved => {
                            handle_cursor_movement(&loop_input_handler, touch.location)
                        }
                        TouchPhase::Started => loop_input_handler.lock().unwrap().set_l_mouse(true),
                        TouchPhase::Ended | TouchPhase::Cancelled => {
                            loop_input_handler.lock().unwrap().set_l_mouse(false)
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        })
        .expect("Error running event loop!");
    *is_running.lock().unwrap() = false;
    if let Err(err) = main_loop.await {
        panic!("An error occured running main loop! {err}");
    }
    runtime.shutdown_background();
}

fn handle_cursor_movement(
    loop_input_handler: &Arc<Mutex<InputHandler>>,
    pos: PhysicalPosition<f64>,
) {
    let vec = Vec2::new(pos.x as f32, pos.y as f32);
    let _ = loop_input_handler
        .lock()
        .unwrap()
        .update_mouse_position(vec);
}
