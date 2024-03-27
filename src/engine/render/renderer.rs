use crate::engine::gui::gui_manager::GuiManager;
use crate::engine::scene::scene::Scene;
use crate::engine::texture::frame_buffer::Framebuffer;
use crate::engine::window_handler::WindowHandler;

use super::background_renderer::BackgroundRenderer;
use super::gui_renderer::GuiRenderer;
use super::postprocess_renderer::PostprocessRenderer;
use super::scene_renderer::SceneRenderer;
use super::uniformbuffer::{MatrixUniformBuffer, UniformBuffer};
use super::updatable::Updatable;

pub struct Renderer {
    matrix_uniform_buffer: UniformBuffer<MatrixUniformBuffer>,
    background_renderer: BackgroundRenderer,
    scene_renderer: SceneRenderer,
    frame_buffer: Framebuffer,
    postprocess_renderer: PostprocessRenderer,
    gui_renderer: GuiRenderer,
}

impl Renderer {
    pub fn init(window_handler: &mut WindowHandler) -> Self {
        gl::load_with(|name| window_handler.get_window_mut().get_proc_address(name));
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::Enable(gl::MULTISAMPLE);
            //gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::PatchParameteri(gl::PATCH_VERTICES, 3);
        };
        let matrix_uniform_buffer;

        unsafe {
            matrix_uniform_buffer = UniformBuffer::create(0);
        }
        Renderer {
            matrix_uniform_buffer,
            background_renderer: BackgroundRenderer::init(),
            scene_renderer: SceneRenderer::init(),
            frame_buffer: Framebuffer::new(window_handler),
            postprocess_renderer: PostprocessRenderer::init(),
            gui_renderer: GuiRenderer::init(),
        }
    }

    pub fn load_scene(&mut self, scene: &Scene, window_handler: &WindowHandler) {
        let matrix_uniform_buffer_content = MatrixUniformBuffer {
            projection_matrix: scene.camera.get_projection_matrix(window_handler),
            view_matrix: scene.camera.get_view_matrix(),
        };
        unsafe {
            self.matrix_uniform_buffer.set_data(matrix_uniform_buffer_content);
        }
        self.postprocess_renderer.load_scene(scene);
    }

    unsafe fn clean_up(&self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
    }

    pub fn update_buffers(&mut self, updatables: &mut Vec<Updatable>, scene: &mut Scene, window_handler: &WindowHandler) {
        while let Some(updatable) = updatables.pop() {
            match updatable {
                Updatable::Projection => {
                    self.frame_buffer.recreate(window_handler);
                    unsafe {
                        self.matrix_uniform_buffer.set_projection_matrix(scene.camera.get_projection_matrix(window_handler));
                    };
                },
                Updatable::View => {
                    unsafe {
                        self.matrix_uniform_buffer.set_view_matrix(scene.camera.get_view_matrix());
                    }
                },
                Updatable::FocalRadius => {
                    self.postprocess_renderer.update_focal_radius(scene);
                },
            }
        }
    }

    pub fn render(&mut self, scene: &Scene, gui_manager: &GuiManager, window_handler: &WindowHandler) {
        unsafe {
            self.clean_up(); // Clean up without framebuffer
            self.frame_buffer.bind();
            self.clean_up(); // Clean up with framebuffer
            gl::Enable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.bind_base();
            self.background_renderer.render(scene);
            self.scene_renderer.render(scene);
            self.frame_buffer.blit(window_handler);
            self.frame_buffer.unbind();
            // Post-processing
            self.postprocess_renderer.render(scene, &self.frame_buffer, window_handler);
            gl::Disable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.unbind_base();
            self.gui_renderer.render(gui_manager, window_handler);
        };
    }
}