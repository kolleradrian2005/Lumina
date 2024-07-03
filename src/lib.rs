extern crate gl;
extern crate glutin;
extern crate glutin_winit;

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

use engine::math::vec2::Vec2;
use engine::references;
use engine::render::renderer::Renderer;
use engine::render::updatable::Updatable;
use engine::scene::scene::Scene;
use engine::texture::resource_manager::ResourceManager;
use engine::{gui::gui_manager::GuiManager, input_handler::InputHandler};

use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use include_assets::{include_dir, NamedArchive};
use rand::{rngs::StdRng, SeedableRng};
use raw_window_handle::HasRawWindowHandle;
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

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
    log::debug!("Starting android app!");
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .unwrap();
    self::start(event_loop);
}

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

#[derive(Clone)]
struct RenderContext {
    pub renderer: Arc<Mutex<Renderer>>,
    pub resource_manager: Arc<Mutex<ResourceManager>>,
    pub scene: Arc<Mutex<Scene>>,
    pub gui_manager: Arc<Mutex<GuiManager>>,
}

impl RenderContext {
    pub fn init(gl_display: &Display, width: i32, height: i32) -> Self {
        let archive = NamedArchive::load(include_dir!("assets"));
        let mut renderer = Renderer::init(gl_display, width, height, &archive);
        let mut resource_manager = ResourceManager::new(archive);
        resource_manager.preload_models();
        resource_manager.load_fonts();
        let scene = Scene::new(&mut resource_manager);
        renderer.load_scene(&scene, width as f32 / height as f32);
        let gui_manager = GuiManager::new((width, height));
        Self {
            renderer: Arc::new(Mutex::new(renderer)),
            resource_manager: Arc::new(Mutex::new(resource_manager)),
            scene: Arc::new(Mutex::new(scene)),
            gui_manager: Arc::new(Mutex::new(gui_manager)),
        }
    }
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

    let updatables_clone = Arc::clone(&updatables);
    let is_running_clone = Arc::clone(&is_running);

    const TARGET_INTERVAL: Duration = Duration::from_millis(10);

    let main_loop = tokio::spawn(async move {
        tokio::time::sleep(TARGET_INTERVAL).await;
        let mut rng: StdRng = SeedableRng::from_entropy();
        let mut delta_time: Duration;
        let mut last: Instant = Instant::now();

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
                    if let Ok(resource_manager) = &mut render_ctx.resource_manager.lock() {
                        scene.update(
                            delta_time.as_secs_f32(),
                            last,
                            loop_input_handler_clone.lock().unwrap().borrow_mut(),
                            resource_manager,
                            &mut rng,
                            updatables_clone.lock().unwrap().borrow_mut(),
                        );
                    }
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
                                if let Ok(resource_manager) = render_ctx.resource_manager.lock() {
                                    gui_manager.update(
                                        &resource_manager,
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
                                        if let Ok(resource_manager) =
                                            &render_ctx.resource_manager.lock()
                                        {
                                            gui_manager.build(
                                                resource_manager,
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
