mod scene_renderer;

use glfw::Window;
use scene_renderer::SceneRenderer;
use crate::scene::Scene;

pub struct Renderer {
    scene_renderer: SceneRenderer
}

impl Renderer {
    pub fn init(window: &mut Window) -> Self {
        gl::load_with(|name| window.get_proc_address(name));
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
        };
        let scene_renderer: SceneRenderer = SceneRenderer::init();
        Renderer { scene_renderer }
    }

    pub fn clean_up(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT)
        };
    }

    pub fn render(&self, scene: &Scene) {
        self.scene_renderer.render(scene);
    }
}