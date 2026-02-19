use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;

use gl::types::{GLint, GLuint};

use crate::render::render_packet::RenderPacket;

use crate::shader::material_parameter::MaterialParameter;
use crate::shader::shader_program::ShaderHandle;
use crate::texture::texture::Texture;

use super::render_entity::RenderEntity;
pub struct GenericRenderer {
    uniform_cache: RefCell<HashMap<GLuint, HashMap<String, GLint>>>,
}

impl GenericRenderer {
    pub fn init() -> Self {
        // Tesselation shaders are not supported on gles 3
        /*let shader_with_tesselation =
        cfg!(not(target_os = "android")).then(|| ModelShader::new(archive, true));*/

        GenericRenderer {
            uniform_cache: RefCell::new(HashMap::new()),
        }
    }

    fn get_uniform_location(
        &self,
        shader_handle: ShaderHandle,
        uniform_name: &str,
    ) -> Option<GLint> {
        let mut cache = self.uniform_cache.borrow_mut();

        let shader_cache = cache.entry(shader_handle.id).or_insert_with(HashMap::new);
        let uniform_location =
            *shader_cache
                .entry(uniform_name.to_string())
                .or_insert_with(|| unsafe {
                    gl::GetUniformLocation(
                        shader_handle.id,
                        std::ffi::CStr::as_ptr(&CString::new(uniform_name.to_string()).unwrap()),
                    )
                });
        if uniform_location == -1 {
            return None;
        }
        Some(uniform_location)
    }

    fn expect_uniform_location(&self, shader_handle: ShaderHandle, uniform_name: &str) -> GLint {
        self.get_uniform_location(shader_handle, uniform_name)
            .expect(&format!(
                "Uniform {} not found in shader with id {}",
                uniform_name, shader_handle.id
            ))
    }

    pub unsafe fn render(&self, render_packet: RenderPacket) {
        for renderable in render_packet.entities.iter() {
            self.render_entity(renderable);
        }

        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }

    pub unsafe fn render_entity(&self, renderable: &RenderEntity) {
        // TODO: optimize by sorting by shader and texture
        let shader_handle = renderable.material.shader;
        gl::UseProgram(shader_handle.id);
        for (name, value) in renderable.material.parameters.iter() {
            let location = self.expect_uniform_location(shader_handle, name);
            match value {
                MaterialParameter::Float(v) => {
                    gl::Uniform1f(location, *v);
                }
                MaterialParameter::Vec2(vec2) => gl::Uniform2f(location, vec2.x, vec2.y),
                MaterialParameter::Vec3(vec3) => gl::Uniform3f(location, vec3.x, vec3.y, vec3.z),
                MaterialParameter::Mat4(matrix) => {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr() as *const f32)
                }
                MaterialParameter::Int(v) => gl::Uniform1i(location, *v),
                MaterialParameter::Bool(v) => gl::Uniform1i(location, *v as i32),
                MaterialParameter::Vec2Array(vec2s) => {
                    gl::Uniform2fv(location, vec2s.len() as i32, vec2s.as_ptr() as *const f32)
                }
            }
        }

        let model_matrix_location: i32 =
            self.expect_uniform_location(shader_handle, "uModelMatrix");
        gl::UniformMatrix4fv(
            model_matrix_location,
            1,
            gl::FALSE,
            renderable.transform_matrix.as_ptr() as *const f32,
        );
        let is_flipped_location: i32 = self.expect_uniform_location(shader_handle, "uFlipped");
        gl::Uniform1i(is_flipped_location, renderable.is_flipped as i32);
        gl::BindVertexArray(renderable.mesh.get_vao());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
        // TODO: handle texture types better e.g. passing array of textures and shader params alongside
        if let Some(texture_type_location) =
            self.get_uniform_location(shader_handle, "uTextureType")
        {
            match &renderable.material.texture {
                Texture::StaticColor(static_color) => {
                    let color_location: i32 = self.expect_uniform_location(shader_handle, "uColor");
                    gl::Uniform3f(
                        color_location,
                        static_color.color.x,
                        static_color.color.y,
                        static_color.color.z,
                    );
                }
                Texture::StaticTexture(static_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, static_texture.get_id());
                }
                Texture::AnimatedTexture(animated_texture) => {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, animated_texture.current_texture().get_id());
                }
                Texture::GradientTexture(_) => {} // Not implemented
            }
            self.get_uniform_location(shader_handle, "uTextureType");
            let value = match renderable.material.texture {
                Texture::StaticColor(_) => 0,
                Texture::StaticTexture(_) => 1,
                Texture::AnimatedTexture(_) => 1,
                Texture::GradientTexture(_) => 2,
            };
            gl::Uniform1i(texture_type_location, value);
        }
        gl::DrawElements(
            match renderable.material.shader.has_tesselation {
                true => gl::PATCHES,
                false => gl::TRIANGLES,
            },
            renderable.mesh.get_vertex_count(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
    }
}
