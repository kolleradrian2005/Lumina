use std::borrow::BorrowMut;
use std::ffi::CString;

use crate::frame_buffer::{Framebuffer, self};
use crate::shader::Shader;
use crate::shader_program::ShaderProgram;
use crate::texture_handler::TextureHandler;
use crate::scene::Scene;
use crate::window_handler;

pub struct PostprocessRenderer {
   shader: ShaderProgram
}

impl PostprocessRenderer {
    pub fn init() -> Self {
        let fragment_shader = Shader::new("postprocess.frag", gl::FRAGMENT_SHADER);
        let vertex_shader = Shader::new("postprocess.vert", gl::VERTEX_SHADER);
        let shader_program = ShaderProgram::new(&[vertex_shader, fragment_shader]);
        shader_program.bind_attributes(0, "position");
        shader_program.bind_attributes(1, "uv");
        PostprocessRenderer {
            shader: shader_program
        }
    }

    pub fn render(&self, scene: &mut Scene, framebuffer: &mut Framebuffer) {
        self.shader.start();
        let model = &mut framebuffer.model;
        let foreground = &mut scene.foreground;
        unsafe {
            let focal_offset = &scene.focal_offset;
            let tint_color_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uTintColor").unwrap()));
            let tint_intensity_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uTintIntensity").unwrap()));
            let focal_radius_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uFocalRadius").unwrap()));
            let darkening_factor_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uDarkeningFactor").unwrap()));
            let aspect_ratio_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uAspectRatio").unwrap()));
            let smooth_factor_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uSmoothFactor").unwrap()));
            let focal_offset_location = gl::GetUniformLocation(self.shader.id, std::ffi::CStr::as_ptr(&CString::new("uFocalOffset").unwrap()));

            gl::Uniform3f(tint_color_location, foreground.tint_color.x, foreground.tint_color.y, foreground.tint_color.z);
            gl::Uniform1f(tint_intensity_location, foreground.tint_intensity);
            gl::Uniform1f(focal_radius_location, foreground.focal_radius);
            gl::Uniform1f(darkening_factor_location, foreground.darkening_factor);
            gl::Uniform1f(aspect_ratio_location, window_handler::WINDOW_WIDTH as f32 / window_handler::WINDOW_HEIGHT as f32);
            gl::Uniform1f(smooth_factor_location, foreground.smooth_factor);
            gl::Uniform2f(focal_offset_location, focal_offset.x, focal_offset.y);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, framebuffer.texture);
            // Draw model
            gl::BindVertexArray(model.get_vao());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::DrawElements(
                gl::TRIANGLES,
                model.get_vertex_count(),
                gl::UNSIGNED_INT,
                0 as * const _
            );            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::BindVertexArray(0);
        };
        self.shader.stop();
    }
}
