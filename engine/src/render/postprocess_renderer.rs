use std::sync::{Arc, Mutex};

use include_assets::NamedArchive;

use crate::scene::foreground::Foreground;
use crate::scene::world::component::camera_component::CameraComponent;
use crate::shader::postprocess_shader::PostprocessShader;
use crate::shader::shader_program_old::ShaderProgram;
use crate::texture::frame_buffer::Framebuffer;

use super::uniformbuffer::{PostProcessUniformBuffer, UniformBuffer};

pub struct PostprocessRenderer {
    shader: PostprocessShader,
    uniform_buffer: UniformBuffer<PostProcessUniformBuffer>,
}

impl PostprocessRenderer {
    pub fn init(archive: &NamedArchive) -> Self {
        unsafe {
            PostprocessRenderer {
                shader: PostprocessShader::new(archive),
                uniform_buffer: UniformBuffer::create(1),
            }
        }
    }

    pub unsafe fn render(
        &mut self,
        camera_component: &CameraComponent,
        foreground: &Foreground,
        framebuffer: &Framebuffer,
    ) {
        self.shader.start();
        self.uniform_buffer.bind_base();

        let mesh = framebuffer.get_mesh();
        self.shader.set_focal_offset(&camera_component.focal_offset);
        self.shader.set_aspect_ratio(framebuffer.get_aspect_ratio());

        let light_positions: Vec<f32> = foreground.get_light_positions();
        let num_lights = light_positions.len() as i32 / 2;
        self.shader.set_num_lights(num_lights);
        self.shader
            .set_light_positions(num_lights, &light_positions);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, framebuffer.get_texture());
        // Draw model
        gl::BindVertexArray(mesh.get_vao());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        gl::DrawElements(
            gl::TRIANGLES,
            mesh.get_vertex_count(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        gl::BindVertexArray(0);

        self.uniform_buffer.unbind_base();
        self.shader.stop();
    }
    pub fn update_focal_radius(&mut self, focal_radius: f32) {
        unsafe { self.uniform_buffer.set_focal_radius(focal_radius) };
    }
    pub fn load_scene(&mut self, foreground: &Arc<Mutex<Foreground>>) {
        unsafe {
            self.uniform_buffer
                .set_data(foreground.lock().unwrap().get_default_uniform_buffer())
        };
    }
}
