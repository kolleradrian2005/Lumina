use std::collections::HashMap;
use std::ffi::CString;

use gl::types::{GLsizeiptr, GLuint, GLvoid};
use glutin::display::{Display, GlDisplay};

use crate::render::resource::texture::texture::StaticTexture;
use crate::render::{frame_buffer::Framebuffer, generic_renderer::GenericRenderer};
use crate::shared::extracted_frame::ExtractedFrame;
use crate::shared::postprocess_config::PostprocessConfig;
use crate::shared::render_entity::RenderEntity;
use crate::shared::window_size::WindowSize;

pub struct Renderer {
    uniform_buffer_pool: HashMap<GLuint, GLuint>,
    frame_buffer: Framebuffer,
    window_size_cache: Option<WindowSize>,
    generic_renderer: GenericRenderer,
}

impl Renderer {
    pub fn init(gl_display: &Display, width: i32, height: i32) -> Self {
        gl::load_with(|name| {
            let symbol = CString::new(name).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

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
        };

        Renderer {
            uniform_buffer_pool: HashMap::new(),
            window_size_cache: None,
            frame_buffer: Framebuffer::new(width, height, msaa),
            generic_renderer: GenericRenderer::init(),
        }
    }

    unsafe fn clean_up(&self) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    pub fn render(&mut self, render_packet: ExtractedFrame) {
        unsafe {
            let postprocess_config: Option<PostprocessConfig> =
                render_packet.postprocess_pass.clone();
            self.refresh_buffers(&render_packet);
            self.initialize_uniformbuffers(&render_packet);
            self.clean_up(); // Clean up without framebuffer
            if postprocess_config.is_some() {
                self.frame_buffer.bind();
                self.clean_up(); // Clean up with framebuffer
            }
            gl::Enable(gl::DEPTH_TEST);
            self.bind_uniform_buffers();
            self.generic_renderer.render(render_packet.entities);
            if postprocess_config.is_some() {
                self.frame_buffer.blit();
                self.frame_buffer.unbind();
            }
            // Post-processing
            if let Some(postprocess_config) = postprocess_config {
                let mut material = postprocess_config.material.clone();
                material.texture = StaticTexture::new(
                    self.frame_buffer.get_texture() as u32,
                    self.frame_buffer.get_width() as u32,
                    self.frame_buffer.get_height() as u32,
                )
                .into();
                self.generic_renderer.render(vec![RenderEntity {
                    mesh: self.frame_buffer.get_mesh(),
                    material,
                    z_index: 0.0,
                }]);
            }
            gl::Disable(gl::DEPTH_TEST);
            self.unbind_uniform_buffers();
        };
    }

    fn refresh_buffers(&mut self, render_packet: &ExtractedFrame) {
        // If window resize => it has to be camera update too
        if let Some(new_window_size) = render_packet.window_size.clone() {
            if self.window_size_cache.is_none()
                || render_packet.window_size != self.window_size_cache
            {
                self.window_size_cache = render_packet.window_size.clone();
                self.frame_buffer
                    .resize(new_window_size.width, new_window_size.height);
                unsafe {
                    gl::Viewport(
                        0,
                        0,
                        new_window_size.width as i32,
                        new_window_size.height as i32,
                    );
                };
            }
        }
    }

    pub fn prepare_frame(&self, frame: &mut ExtractedFrame) {
        frame
            .entities
            .sort_by(|a, b| a.z_index.total_cmp(&b.z_index));
        // TODO: Sort by shader, texture, material, etc. to minimize state changes and create a PreparedFrame
    }

    fn initialize_uniformbuffers(&mut self, render_packet: &ExtractedFrame) {
        render_packet
            .uniform_buffers
            .iter()
            .for_each(|uniform_buffer_data| {
                if let Some(ubo) = self
                    .uniform_buffer_pool
                    .get(&uniform_buffer_data.binding_index)
                {
                    unsafe {
                        gl::BindBuffer(gl::UNIFORM_BUFFER, *ubo);
                        gl::BufferSubData(
                            gl::UNIFORM_BUFFER,
                            0,
                            uniform_buffer_data.data.len() as GLsizeiptr,
                            uniform_buffer_data.data.as_ptr() as *const GLvoid,
                        );
                        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
                    }
                } else {
                    let mut ubo: GLuint = 0;
                    unsafe {
                        gl::GenBuffers(1, &mut ubo);
                        gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);
                        gl::BufferData(
                            gl::UNIFORM_BUFFER,
                            uniform_buffer_data.data.len() as GLsizeiptr,
                            uniform_buffer_data.data.as_ptr() as *const GLvoid,
                            gl::DYNAMIC_DRAW,
                        );
                        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
                    };
                    self.uniform_buffer_pool
                        .insert(uniform_buffer_data.binding_index, ubo);
                }
            });
    }

    fn bind_uniform_buffers(&self) {
        for (binding_index, ubo) in self.uniform_buffer_pool.iter() {
            unsafe {
                gl::BindBufferBase(gl::UNIFORM_BUFFER, *binding_index, *ubo);
            }
        }
    }

    fn unbind_uniform_buffers(&self) {
        for (binding_index, _) in self.uniform_buffer_pool.iter() {
            unsafe {
                gl::BindBufferBase(gl::UNIFORM_BUFFER, *binding_index, 0);
            }
        }
    }
}
