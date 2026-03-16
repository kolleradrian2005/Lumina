use gl::types::GLuint;

use crate::render::resource::shader::parameter_schema::ParameterSchema;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShaderProgramHandle {
    pub id: GLuint,
    pub has_tesselation: bool,
}

pub struct ShaderProgram {
    handle: ShaderProgramHandle,
    //uniform_locations: HashMap<String, GLint>,
    parameter_schema: ParameterSchema,
}

impl ShaderProgram {
    pub fn new(handle: ShaderProgramHandle, parameter_schema: ParameterSchema) -> Self {
        Self {
            handle,
            parameter_schema,
        }
    }

    pub fn get_handle(&self) -> ShaderProgramHandle {
        self.handle
    }

    pub fn get_parameter_schema(&self) -> &ParameterSchema {
        &self.parameter_schema
    }
}
