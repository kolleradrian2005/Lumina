use gl::types::*;

use crate::shader::Shader;

pub struct ShaderProgram {
    pub id: GLuint
}

impl ShaderProgram {
    pub fn new(shaders: &[Shader]) -> Self {
        unsafe {
            let program = Self {
                id: gl::CreateProgram()
            };
            for shader in shaders {
                gl::AttachShader(program.id, shader.id);
            }
            gl::LinkProgram(program.id);
            let mut success = 0;
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
            if success == gl::FALSE as i32 {
                let mut error_log_size = 0;
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
            program.id,
            error_log_size,
            &mut error_log_size,
            error_log.as_mut_ptr() as *mut _
                );
                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log);
                panic!("{}", log.unwrap());
            }
            program
        }
    }
    pub fn bind_attributes(&self, attribute: u32, variable_name: &str) {
        unsafe { gl::BindAttribLocation(self.id, attribute, variable_name.as_bytes().as_ptr() as *const GLchar) }
    }
    pub fn start(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
    pub fn stop(&self) {
        unsafe { gl::UseProgram(0) };
    }
}
