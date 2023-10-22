use glfw::Window;
use crate::background_renderer::BackgroundRenderer;
use crate::frame_buffer::Framebuffer;
use crate::player_renderer::PlayerRenderer;
use crate::postprocess_renderer::PostprocessRenderer;
use crate::scene_renderer::SceneRenderer;
use crate::scene::Scene;
use crate::terrain_renderer::TerrainRenderer;
use crate::texture_handler::TextureHandler;

pub struct Renderer {
    background_renderer: BackgroundRenderer,
    terrain_renderer: TerrainRenderer,
    scene_renderer: SceneRenderer,
    player_renderer: PlayerRenderer,
    frame_buffer: Framebuffer,
    postprocess_renderer: PostprocessRenderer
}

impl Renderer {
    pub fn init(window: &mut Window) -> Self {
        gl::load_with(|name| window.get_proc_address(name));
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        };
        let background_renderer: BackgroundRenderer = BackgroundRenderer::init();
        let terrain_renderer: TerrainRenderer = TerrainRenderer::init();
        let scene_renderer: SceneRenderer = SceneRenderer::init();
        let player_renderer: PlayerRenderer = PlayerRenderer::init();
        let frame_buffer: Framebuffer = Framebuffer::new();
        let postprocess_renderer: PostprocessRenderer = PostprocessRenderer::init();
        Renderer {
            background_renderer,
            terrain_renderer,
            scene_renderer,
            player_renderer,
            frame_buffer,
            postprocess_renderer 
        }
    }

    pub fn clean_up(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT )
        };
    }

    pub fn render(&mut self, scene: &mut Scene, texture_handler: &mut TextureHandler) {
        self.frame_buffer.bind();
        self.background_renderer.render(scene, texture_handler);
        self.terrain_renderer.render(scene);
        self.scene_renderer.render(scene, texture_handler);
        self.player_renderer.render(scene, texture_handler);
        self.frame_buffer.unbind();
        // Post-processing
        self.postprocess_renderer.render(scene, &mut self.frame_buffer);
    }
}