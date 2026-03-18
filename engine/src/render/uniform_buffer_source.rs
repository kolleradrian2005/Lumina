use gl::types::{GLsizeiptr, GLuint, GLvoid};

use crate::shared::uniform_buffer_render_data::UniformBufferRenderData;

pub struct UniformBufferSource<T: Copy> {
    binding_index: GLuint,
    pub data: T,
}

impl<T: Copy> UniformBufferSource<T> {
    pub fn new(binding_index: GLuint, data: T) -> Self {
        Self {
            binding_index,
            data,
        }
    }

    pub fn update(&mut self, new_data: T) {
        self.data = new_data;
    }

    pub fn extract(&self) -> UniformBufferRenderData {
        let data_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.data as *const T as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        UniformBufferRenderData {
            binding_index: self.binding_index,
            data: data_bytes.to_vec(),
        }
    }
}

impl UniformBufferRenderData {
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::UNIFORM_BUFFER, self.binding_index) };
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::UNIFORM_BUFFER, 0) };
    }

    pub fn generate_buffer(&self) -> GLuint {
        let mut ubo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ubo) };
        ubo
    }

    pub fn initialize_buffer_data(&self) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                self.data.len() as GLsizeiptr,
                &self.data as *const _ as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
        self.unbind();
    }

    pub fn refresh_buffer_data(&self) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                self.data.len() as GLsizeiptr,
                &self.data as *const _ as *const GLvoid,
            );
        }
        self.unbind();
    }
}
