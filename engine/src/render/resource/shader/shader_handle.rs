use gl::types::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShaderHandle {
    pub id: GLuint,
}
