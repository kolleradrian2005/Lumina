use gl::types::GLuint;

// Asynchronously called GL commands
pub enum GlCommand {
    DeleteVao(GLuint),
    DeleteBuffer(GLuint),
}

impl GlCommand {
    pub fn execute(&self) {
        unsafe {
            match self {
                GlCommand::DeleteVao(vao) => gl::DeleteVertexArrays(1, vao),
                GlCommand::DeleteBuffer(vbo) => gl::DeleteBuffers(1, vbo),
            }
        }
    }
}
