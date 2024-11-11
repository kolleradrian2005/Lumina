use std::collections::VecDeque;
use std::ffi::CString;
use std::sync::Arc;

use glutin::display::{Display, GlDisplay};
use include_assets::NamedArchive;

use crate::engine::command_queue::CommandQueue;
use crate::engine::gui::gui_manager::GuiManager;
use crate::engine::scene::scene::Scene;
use crate::engine::texture::frame_buffer::Framebuffer;

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
    pub fn init(
        command_queue: Arc<CommandQueue>,
        gl_display: &Display,
        width: i32,
        height: i32,
        archive: &NamedArchive,
    ) -> Self {
        gl::load_with(|name| {
            let symbol = CString::new(name).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        let matrix_uniform_buffer;
        let msaa = match cfg!(not(target_os = "android")) {
            true => Some(16),
            false => None,
        };

        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);

            if msaa.is_some() {
                gl::Enable(gl::MULTISAMPLE);
            }

            gl::DepthFunc(gl::LEQUAL);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::PatchParameteri(gl::PATCH_VERTICES, 3);

            matrix_uniform_buffer = UniformBuffer::create(0);
        };

        Renderer {
            matrix_uniform_buffer,
            background_renderer: BackgroundRenderer::init(archive),
            scene_renderer: SceneRenderer::init(archive),
            frame_buffer: Framebuffer::new(command_queue, width, height, msaa),
            postprocess_renderer: PostprocessRenderer::init(archive),
            gui_renderer: GuiRenderer::init(archive),
        }
    }

    pub fn load_scene(&mut self, scene: &Scene, aspect_ratio: f32) {
        let matrix_uniform_buffer_content = MatrixUniformBuffer {
            projection_matrix: scene.camera.get_projection_matrix(aspect_ratio),
            view_matrix: scene.camera.get_view_matrix(),
        };
        unsafe {
            self.matrix_uniform_buffer
                .set_data(matrix_uniform_buffer_content);
        }
        self.postprocess_renderer.load_scene(scene);
    }

    unsafe fn clean_up(&self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    pub fn update_buffers(
        &mut self,
        command_queue: Arc<CommandQueue>,
        updatables: &mut VecDeque<Updatable>,
        scene: &mut Scene,
    ) {
        while let Some(updatable) = updatables.pop_front() {
            match updatable {
                Updatable::Projection { width, height } => {
                    self.frame_buffer
                        .resize(command_queue.clone(), width, height);
                    unsafe {
                        gl::Viewport(0, 0, width as i32, height as i32);
                        self.matrix_uniform_buffer.set_projection_matrix(
                            scene
                                .camera
                                .get_projection_matrix(width as f32 / height as f32),
                        );
                    };
                }
                Updatable::View => unsafe {
                    self.matrix_uniform_buffer
                        .set_view_matrix(scene.camera.get_view_matrix());
                },
                Updatable::FocalRadius => {
                    self.postprocess_renderer.update_focal_radius(scene);
                }
            }
        }
    }

    pub fn render(
        &mut self,
        command_queue: Arc<CommandQueue>,
        scene: &mut Scene,
        gui_manager: &GuiManager,
    ) {
        unsafe {
            self.clean_up(); // Clean up without framebuffer
            self.frame_buffer.bind();
            self.clean_up(); // Clean up with framebuffer
            gl::Enable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.bind_base();
            self.background_renderer.render(scene);
            self.scene_renderer.render(command_queue, scene);
            self.frame_buffer.blit();
            self.frame_buffer.unbind();
            // Post-processing
            self.postprocess_renderer.render(scene, &self.frame_buffer);
            gl::Disable(gl::DEPTH_TEST);
            self.matrix_uniform_buffer.unbind_base();
            self.gui_renderer
                .render(gui_manager, self.frame_buffer.get_aspect_ratio());
        };
    }
}
