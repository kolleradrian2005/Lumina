use gl::types::GLuint;

pub trait ShaderProgram {
    fn get_id(&self) -> GLuint;
    unsafe fn start(&self);
    unsafe fn stop(&self);
}