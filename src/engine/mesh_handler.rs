use gl::types::*;

pub struct MeshHandler;

impl MeshHandler {
    pub fn create_mesh(&self, vertices: &[f32], indices: &[u32], uvs: &[f32]) -> GLuint {
        let mut vao: GLuint = 0;
        unsafe {
            // Generate VAO id
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            // Store attributes
            self.store_data(0, 3, vertices);
            self.store_data(1, 2, uvs);
            self.bind_indices(indices);
            // Unbind VAO
            gl::BindVertexArray(0);
        };
        vao
    }

    fn store_data(&self, attribute: u32, dimensions: i32, data: &[f32]) {
        unsafe {
            let mut vbo: GLuint = 0;
            // Generate VBO
            gl::GenBuffers(1, &mut vbo);
            // Bind VBO
            gl::BindBuffer(gl::ARRAY_BUFFER , vbo);
            // Buffer data
            let float_size = std::mem::size_of::<f32>();
            let data_size = data.len() * float_size;
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data_size as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
            gl::VertexAttribPointer(
                attribute,
                dimensions,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );
            // Unbind VBO
            //gl::BindBuffer(gl::ARRAY_BUFFER , 0);
        };
    }

    fn bind_indices(&self, indices: &[u32]) {
        unsafe {
            let mut ebo: GLuint = 0;
            // Generate EBO
            gl::GenBuffers(1, &mut ebo);
            // Bind EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER , ebo);
            // Buffer data
            let uint_size = std::mem::size_of::<u32>();
            let data_size = indices.len() * uint_size;
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                data_size as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
            // Unbind EBO
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER , 0);
        }
    }
}