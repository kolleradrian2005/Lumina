use std::sync::Arc;

use gl::types::{GLsizei, GLsizeiptr, GLuint, GLvoid};

use crate::engine::{command_queue::CommandQueue, gl_command::GlCommand};

#[derive(Clone, Debug)]
pub struct Mesh {
    vao: GLuint,
    vert_vbo: GLuint,
    uvs_vbo: Option<GLuint>,
    ebo: GLuint,
    vertex_count: GLsizei,
    command_queue: Arc<CommandQueue>,
}

impl Mesh {
    pub fn new(
        command_queue: Arc<CommandQueue>,
        vertices: &[f32],
        indices: &[u32],
        uvs: &[f32],
    ) -> Self {
        let mut vao: GLuint = 0;
        let mut uvs_vbo = None;
        unsafe {
            // Generate VAO id
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            // Store attributes
            let vert_vbo = Self::store_data(0, 3, vertices);
            if 0 < uvs.len() {
                uvs_vbo = Self::store_data(1, 2, uvs).into();
            }
            let ebo = Self::bind_indices(indices);
            // Unbind VAO
            gl::BindVertexArray(0);
            return Self {
                vao,
                vert_vbo,
                uvs_vbo,
                ebo,
                vertex_count: indices.len() as GLsizei,
                command_queue,
            };
        };
    }

    fn store_data(attribute: u32, dimensions: i32, data: &[f32]) -> GLuint {
        let mut vbo: GLuint = 0;
        unsafe {
            // Generate VBO
            gl::GenBuffers(1, &mut vbo);
            // Bind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // Buffer data
            let float_size = std::mem::size_of::<f32>();
            let data_size = data.len() * float_size;
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data_size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(
                attribute,
                dimensions,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
            // Unbind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        };
        return vbo;
    }

    fn bind_indices(indices: &[u32]) -> GLuint {
        let mut ebo: GLuint = 0;
        unsafe {
            // Generate EBO
            gl::GenBuffers(1, &mut ebo);
            // Bind EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // Buffer data
            let uint_size = std::mem::size_of::<u32>();
            let data_size = indices.len() * uint_size;
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                data_size as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            // Unbind EBO
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER , 0);
        }
        return ebo;
    }

    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub fn get_vertex_count(&self) -> GLsizei {
        self.vertex_count
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        let sender = self.command_queue.get_sender();
        sender.send(GlCommand::DeleteVao(self.vao)).unwrap();
        sender.send(GlCommand::DeleteBuffer(self.vert_vbo)).unwrap();
        if let Some(uvs_vbo) = self.uvs_vbo {
            sender.send(GlCommand::DeleteBuffer(uvs_vbo)).unwrap();
        }
        sender.send(GlCommand::DeleteBuffer(self.ebo)).unwrap();
    }
}
