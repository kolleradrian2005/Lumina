use flume::Sender;
use include_assets::{include_dir, NamedArchive};

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
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowBuilder};

use crate::engine_config;
use crate::logic::engine_logic::run_logic_loop;
use crate::logic::scene::scene::Scene;
use crate::math::vec2::Vec2;
use crate::render::renderer::Renderer;
use crate::render::resource::resource_loader::ResourceLoader;
use crate::render::resource::resource_manager::ResourceManager;
use crate::render::resource::resource_provider::ResourceProvider;
use crate::shared::input::input_event::InputEvent;

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
        .with_title(engine_config::WINDOW_TITLE)
        .with_inner_size(PhysicalSize::new(
            engine_config::INITIAL_WINDOW_WIDTH,
            engine_config::INITIAL_WINDOW_HEIGHT,
        ))
}

pub fn start(on_init: impl FnMut(&mut Scene, &mut ResourceManager) + Send + 'static) {
    let event_loop = EventLoopBuilder::new()
        .build()
        .expect("Failed to create event loop!");
    start_with_event_loop(event_loop, on_init);
}

#[cfg(target_os = "android")]
pub fn start_with_android_app(
    app: winit::platform::android::activity::AndroidApp,
    on_init: impl FnMut(&mut Scene, &mut ResourceManager) + Send + 'static,
) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let event_loop = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .expect("Failed to create Android event loop!");
    start_with_event_loop(event_loop, on_init);
}

fn start_with_event_loop(
    event_loop: EventLoop<()>,
    mut on_init: impl FnMut(&mut Scene, &mut ResourceManager) + Send + 'static,
) {
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

    // Define channels
    let (input_tx, input_rx) = flume::unbounded();
    let (render_tx, render_rx) = flume::bounded(2);
    let (resource_tx, resource_rx) = flume::bounded(2);

    // Delegate logic to separate thread
    let _logic_handle = std::thread::spawn(move || {
        let mut scene = Scene::new();
        let mut resource_manager = ResourceManager::new(resource_tx.clone());
        resource_manager.attach_archive(NamedArchive::load(include_dir!("assets")));
        resource_manager.load_default_meshes();
        resource_manager.load_default_shaders();
        on_init(&mut scene, &mut resource_manager);
        scene.get_world_mut().insert_resource(resource_manager);
        run_logic_loop(input_rx, render_tx, scene);
    });

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
                    renderer = Renderer::init(&gl_display, width, height).into();
                    resource_loader = ResourceLoader::new(resource_rx.clone()).into();
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
                            if let Ok(mut packet) = render_rx.try_recv() {
                                renderer.prepare_frame(&mut packet);
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
