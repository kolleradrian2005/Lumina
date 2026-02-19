extern crate gl;
extern crate glutin;
extern crate glutin_winit;

/* For later implementation when gui is added back in
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
}*/
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
    pub mod generic_renderer;
    pub mod render_entity;
    pub mod render_packet;
    pub mod renderer;
    pub mod uniformbuffer;
    pub mod updatable;
}
pub mod scene {
    pub mod world {
        pub mod entity {
            pub mod entity;
            pub mod particle_entity;
        }
        pub mod component {
            pub mod camera_component;
            pub mod collider_component;
            pub mod component;
            pub mod current_component;
            pub mod emitter_component;
            pub mod force_component;
            pub mod material_component;
            pub mod model_component;
            pub mod movement_component;
            pub mod movement_stats_component;
            pub mod parent_component;
            pub mod shader_params_component;
            pub mod transform_component;
        }
        pub mod system {
            pub mod collision_system;
            pub mod emitter_system;
            pub mod movement_system;
            pub mod particle_system;
            pub mod render_system;
            pub mod system;
        }
        pub mod component_storage;
        pub mod create_mesh_manager;
        pub mod drop_mesh_request;
        pub mod query;
        pub mod world;
    }
    pub mod foreground;
    pub mod scene;
}
pub mod shader {
    pub mod material_parameter;
    pub mod parameter_schema;
    pub mod shader;
    pub mod shader_configuration;
    pub mod shader_handler;
    pub mod shader_parameter_type;
    pub mod shader_program;
}
pub mod texture {
    //pub mod font_texture;
    pub mod frame_buffer;
    pub mod resource_command;
    pub mod resource_loader;
    pub mod resource_manager;
    pub mod resource_provider;
    pub mod texture;
    pub mod texture_handler;
}
pub mod input {
    pub mod input_event;
    pub mod input_state;
}
pub mod collider;
pub mod logic;
pub mod references;
pub mod transformable;

use flume::Sender;
use include_assets::{include_dir, NamedArchive};
use math::vec2::Vec2;

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;

use std::num::NonZeroU32;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{
    DeviceEvent, ElementState, Event, KeyEvent, MouseButton, TouchPhase, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowBuilder};

use crate::input::input_event::InputEvent;
use crate::logic::run_logic_loop;
use crate::render::renderer::Renderer;
use crate::scene::scene::Scene;
use crate::texture::resource_loader::ResourceLoader;
use crate::texture::resource_manager::ResourceManager;
use crate::texture::resource_provider::ResourceProvider;

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
        .expect("No valid standard OpenGL config found!")
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
pub async fn start(
    event_loop: EventLoop<()>,
    mut on_init: impl FnMut(&mut Scene, &mut ResourceManager) + Send + 'static,
) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
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

    let (input_tx, input_rx) = flume::unbounded();
    let (render_tx, render_rx) = flume::bounded(2);
    let (resource_tx, resource_rx) = flume::bounded(2);
    let _logic_handle = std::thread::spawn(move || {
        let mut scene = Scene::new();
        let mut resource_manager = ResourceManager::new(resource_tx.clone());
        resource_manager.attach_archive(NamedArchive::load(include_dir!("assets")));
        resource_manager.load_default_models();
        resource_manager.load_default_shaders();
        on_init(&mut scene, &mut resource_manager);
        scene.get_world_mut().insert_resource(resource_manager);
        runtime.block_on(run_logic_loop(input_rx, render_tx, scene));
    });
    /*
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
                    world.insert_resource(loop_input_state_clone.clone());
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
    */

    let mut renderer: Option<Renderer> = None;
    let mut resource_loader: Option<ResourceLoader> = None;
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
                    /*let mut render_ctx = RenderContext::init(&gl_display, width, height);
                    on_render_ctx_init(&mut render_ctx);
                    render_context.get_or_insert_with(|| render_ctx.clone());
                    loop_render_context
                        .lock()
                        .unwrap()
                        .get_or_insert_with(|| render_ctx);*/
                    renderer = Renderer::init(&gl_display, width, height).into();
                    resource_loader = ResourceLoader::new(resource_rx.clone()).into();
                    //resource_loader.load_default_models();
                    //resource_loader.load_fonts();

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
                    DeviceEvent::Button { button, state } => {
                        let mouse_event: Option<InputEvent> = match button {
                            0 => InputEvent::MouseEvent {
                                button: MouseButton::Left,
                                pressed: state.is_pressed(),
                            }
                            .into(),
                            1 => InputEvent::MouseEvent {
                                button: MouseButton::Right,
                                pressed: state.is_pressed(),
                            }
                            .into(),
                            _ => None,
                        };
                        if let Some(mouse_event) = mouse_event {
                            let _ = input_tx.send(mouse_event);
                        }
                    }
                    _ => {}
                },
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::RedrawRequested => {
                        if let (Some(renderer), Some(resource_loader)) =
                            (&mut renderer, &mut resource_loader)
                        {
                            resource_loader.run();
                            if let Ok(packet) = render_rx.try_recv() {
                                // Handle ui logic
                                /*if let Ok(gui_manager) = &mut render_ctx.gui_manager.lock() {
                                    if let Ok(resource_provider) = render_ctx.resource_handle.lock() {
                                        gui_manager.update(
                                            &*resource_provider,
                                            loop_input_state.lock().unwrap().borrow_mut(),
                                        );
                                    }
                                }*/

                                // Update buffers based on scene
                                /*if let Ok(updatables) = &mut updatables.lock() {
                                                                renderer.update_buffers(updatables, scene);
                                                            }
                                                            //if let Ok(gui_manager) = &mut render_ctx.gui_manager.lock() {
                                                            let drop_mesh_requests = &mut drop_mesh_requests.lock().unwrap();
                                                            let create_mesh_manager = &mut create_mesh_manager.lock().unwrap();
                                                            renderer.handle_mesh_requests(drop_mesh_requests, create_mesh_manager);
                                                            let background =
                                                                scene.get_world_mut().expect_resource_ptr::<Background>();
                                                            let foreground_lock = &scene
                                                                .get_world()
                                                                .expect_resource::<Arc<Mutex<Foreground>>>()
                                                                .clone();
                                                            let foreground = foreground_lock.lock().unwrap();
                                                            let (_camera, (camera_component,)) = scene
                                                                .get_world()
                                                                .query::<(&CameraComponent,)>()
                                                                .next()
                                                                .expect("No camera found in the scene")
                                                                .clone();
                                */
                                renderer.render(packet);

                                if let Some((_, _, window)) = &state {
                                    window.pre_present_notify();
                                }
                                if let Some((gl_context, gl_surface, _)) = &state {
                                    gl_surface
                                        .swap_buffers(gl_context)
                                        .expect("Error swapping buffers!");
                                }
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
                                let _ = input_tx.send(InputEvent::WindowResize { width, height });

                                /* For later implementation when gui is added back in
                                    gui_manager.resize((width, height);
                                    gui_manager.build(
                                        &**resource_provider,
                                        width as f32 / height as f32,
                                    );
                                */
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
                            ElementState::Pressed => InputEvent::KeyDown(logical_key.clone()),
                            ElementState::Released => InputEvent::KeyUp(logical_key.clone()),
                        };
                        let _ = input_tx.send(new_state);
                    }

                    WindowEvent::CursorMoved {
                        device_id: _,
                        position: pos,
                    } => handle_cursor_movement(&input_tx, pos),
                    WindowEvent::Touch(touch) => match touch.phase {
                        TouchPhase::Moved => handle_cursor_movement(&input_tx, touch.location),
                        TouchPhase::Started => {
                            let _ = input_tx.send(InputEvent::MouseEvent {
                                button: MouseButton::Left,
                                pressed: true,
                            });
                        }
                        TouchPhase::Ended | TouchPhase::Cancelled => {
                            let _ = input_tx.send(InputEvent::MouseEvent {
                                button: MouseButton::Left,
                                pressed: false,
                            });
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        })
        .expect("Error running event loop!");
}

fn handle_cursor_movement(input_tx: &Sender<InputEvent>, pos: PhysicalPosition<f64>) {
    let vec = Vec2::new(pos.x as f32, pos.y as f32);
    let _ = input_tx.send(InputEvent::MouseMove(vec));
}
