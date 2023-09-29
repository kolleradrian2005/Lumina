mod scene_renderer;

use glfw::Window;
use crate::player_renderer::PlayerRenderer;
use crate::scene_renderer::SceneRenderer;
use crate::scene::Scene;
use crate::texture_handler::TextureHandler;


pub struct Renderer {
    scene_renderer: SceneRenderer,
    player_renderer: PlayerRenderer
}

impl Renderer {
    pub fn init(window: &mut Window) -> Self {
        gl::load_with(|name| window.get_proc_address(name));
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        };
        let scene_renderer: SceneRenderer = SceneRenderer::init();
        let player_renderer: PlayerRenderer = PlayerRenderer::init();
        Renderer { scene_renderer, player_renderer }
    }

    pub fn clean_up(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT)
        };
    }

    pub fn render(&self, scene: &mut Scene, texture_handler: &mut TextureHandler) {
        self.scene_renderer.render(scene, texture_handler);
        self.player_renderer.render(scene, texture_handler);
    }
}