use std::fmt::Debug;

use gl::types::GLuint;

use crate::shader::parameter_schema::ParameterSchema;

pub trait ShaderProgramOld: Send + Sync {
    fn get_id(&self) -> GLuint;
    unsafe fn start(&self);
    unsafe fn stop(&self);
    fn get_parameter_schema(&self) -> ParameterSchema;
}

impl Debug for dyn ShaderProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ShaderProgram {{ id: {}, parameter_schema: {:?} }}",
            self.get_id(),
            self.get_parameter_schema()
        )
    }
}
