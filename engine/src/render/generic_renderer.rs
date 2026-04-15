use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;

use gl::types::{GLint, GLuint};

use crate::{
    logic::ecs::component::material::DrawMode,
    render::{
        resource::shader::{
            material_parameter::MaterialParameter, shader_program::ShaderProgramHandle,
        },
        resource::texture::texture::Texture,
    },
    shared::render_entity::RenderEntity,
};

pub struct GenericRenderer {
    uniform_cache: RefCell<HashMap<GLuint, HashMap<String, GLint>>>,
}

impl GenericRenderer {
    pub fn init() -> Self {
        GenericRenderer {
            uniform_cache: RefCell::new(HashMap::new()),
        }
    }

    fn get_uniform_location(
        &self,
        shader_handle: ShaderProgramHandle,
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

    fn expect_uniform_location(
        &self,
        shader_handle: ShaderProgramHandle,
        uniform_name: &str,
    ) -> GLint {
        self.get_uniform_location(shader_handle, uniform_name)
            .expect(&format!(
                "Uniform {} not found in shader with id {}",
                uniform_name, shader_handle.id
            ))
    }

    pub unsafe fn render(&self, entities: Vec<RenderEntity>) {
        let mut last_shader: GLuint = 0;
        let mut last_vao: GLuint = 0;
        for renderable in entities {
            self.render_entity(&renderable, &mut last_shader, &mut last_vao);
        }

        gl::DisableVertexAttribArray(0);
        gl::DisableVertexAttribArray(1);
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }

    pub unsafe fn render_entity(
        &self,
        renderable: &RenderEntity,
        last_shader: &mut GLuint,
        last_vao: &mut GLuint,
    ) {
        let shader_handle = renderable.material.shader;
        if shader_handle.id != *last_shader {
            gl::UseProgram(shader_handle.id);
            *last_shader = shader_handle.id;
        }
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
        let vao = renderable.mesh.get_vao();
        if vao != *last_vao {
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(0);
            if renderable.mesh.get_uvs_vbo().is_some() {
                gl::EnableVertexAttribArray(1);
            } else {
                gl::DisableVertexAttribArray(1);
            }
            *last_vao = vao;
        }

        // TODO: handle texture types better e.g. passing array of textures and shader params alongside

        match &renderable.material.texture {
            Texture::None => {}
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
            Texture::GradientTexture(_) => {}
        }
        if let Some(texture_type_location) =
            self.get_uniform_location(shader_handle, "uTextureType")
        {
            let value = match renderable.material.texture {
                Texture::StaticColor(_) => 0,
                Texture::StaticTexture(_) => 1,
                Texture::AnimatedTexture(_) => 1,
                Texture::GradientTexture(_) => 2,
                Texture::None => 3,
            };
            gl::Uniform1i(texture_type_location, value);
        }
        gl::DrawElements(
            match renderable.material.draw_mode {
                DrawMode::Triangles => gl::TRIANGLES,
                DrawMode::Lines => gl::LINES,
                DrawMode::Patches => gl::PATCHES,
            },
            renderable.mesh.get_vertex_count(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
        gl_check_error!();
    }
}
