use gl::types::*;

use super::{shader::Shader, shader_program::ShaderProgram};

pub fn load_program(shaders: &[Shader]) -> GLuint {
    unsafe {
        let id = gl::CreateProgram();
        for shader in shaders {
            gl::AttachShader(id, shader.id);
        }
        gl::LinkProgram(id);
        let mut success = 0;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as i32 {
            let mut error_log_size = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
        id,
        error_log_size,
        &mut error_log_size,
        error_log.as_mut_ptr() as *mut _
            );
            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log);
            panic!("{}", log.unwrap());
        }
        id
    }
}

pub unsafe fn bind_attributes_to_program(shader_program: &dyn ShaderProgram, attribute: u32, variable_name: &str) {
    gl::BindAttribLocation(shader_program.get_id(), attribute, variable_name.as_bytes().as_ptr() as *const GLchar);
}

pub unsafe fn start_program(shader_program: &dyn ShaderProgram) {
    gl::UseProgram(shader_program.get_id());
}

pub unsafe fn stop_program() {
    gl::UseProgram(0);
}

