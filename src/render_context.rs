use std::sync::{Arc, Mutex};

use glutin::display::Display;
use include_assets::{include_dir, NamedArchive};

use crate::engine::{
    gui::gui_manager::GuiManager,
    render::renderer::Renderer,
    scene::{foreground::Foreground, scene::Scene},
    texture::{
        resource_manager::ResourceManager,
        resource_provider::{ResourceProvider, ResourceProviderHandle},
    },
};

#[derive(Clone)]
pub struct RenderContext {
    pub renderer: Arc<Mutex<Renderer>>,
    pub resource_handle: ResourceProviderHandle,
    pub scene: Arc<Mutex<Scene>>,
    pub gui_manager: Arc<Mutex<GuiManager>>,
}

impl RenderContext {
    pub fn init(gl_display: &Display, width: i32, height: i32) -> Self {
        let archive = NamedArchive::load(include_dir!("assets"));
        let mut renderer = Renderer::init(gl_display, width, height, &archive);
        let mut resource_provider = ResourceManager::new(archive);
        resource_provider.preload_models();
        resource_provider.load_fonts();
        let mut scene = Scene::new(&mut resource_provider);
        scene
            .get_world_mut()
            .insert_resource(Arc::new(Mutex::new(Foreground::construct())));
        renderer.load_scene(&scene, width as f32 / height as f32);
        let gui_manager = GuiManager::new((width, height));
        Self {
            renderer: Arc::new(Mutex::new(renderer)),
            resource_handle: ResourceProviderHandle::new(resource_provider),
            scene: Arc::new(Mutex::new(scene)),
            gui_manager: Arc::new(Mutex::new(gui_manager)),
        }
    }
}
