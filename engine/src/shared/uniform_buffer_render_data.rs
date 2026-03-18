use gl::types::GLuint;

#[derive(Clone, Debug)]
pub struct UniformBufferRenderData {
    pub binding_index: GLuint,
    pub data: Vec<u8>,
}
