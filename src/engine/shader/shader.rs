use std::{fs, ffi::CString};

use gl::types::*;

use crate::engine::references;

pub struct Shader {
    pub id: GLuint
}

impl Shader {
    pub fn new(shader_name: &str, shader_type: GLenum) -> Self {
        unsafe {
            let path = String::new() + references::ASSETS_PATH + shader_name;
            let contents = fs::read_to_string(&path);
            if let Err(err) = &contents {
                panic!("{} {}", err.to_string(), path);
            }
            let source = CString::new(contents.unwrap());
            if source.is_err() {
                panic!("Unable to create CString from file");
            }
            let shader = Self {
                id: gl::CreateShader(shader_type)
            };
            gl::ShaderSource(shader.id, 1, &std::ffi::CStr::as_ptr(&source.unwrap()), std::ptr::null());
            gl::CompileShader(shader.id);
            let mut success = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint {
                let mut error_log_size = 0;
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = vec!(0u8; error_log_size as usize);
                gl::GetShaderInfoLog(shader.id, error_log_size, std::ptr::null_mut(), error_log.as_mut_ptr() as *mut GLchar);
                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8_lossy(&error_log);
                println!("Could not compile: {} Log: {:?}", path, log);
                panic!("{:?}", log);
            }
            shader
        }
    }
}
