use std::{ffi::CString, io::Read, path::Path};

use gl::types::*;
use include_assets::NamedArchive;

use crate::references;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(archive: &NamedArchive, shader_name: &str, shader_type: GLenum) -> Option<Self> {
        unsafe {
            let path = Path::new(references::SHADERS_PATH).join(shader_name.replace("/", "\\"));
            let binding = path.to_string_lossy().replace("/", "\\");
            let path_str = binding.as_str();

            let mut asset = archive.get(path_str);
            if asset.is_none() {
                println!("Unable to find shader: {}", path_str);
                return None;
            }
            let asset = asset.as_mut().unwrap();
            let mut contents = String::new();

            if let Err(err) = &asset.read_to_string(&mut contents) {
                panic!("{} {}", err.to_string(), path_str);
            }

            let mut source_raw = String::from(references::SHADER_VERSION_HEADER);

            if cfg!(target_os = "android") {
                source_raw.push_str("#define ES\r\n"); // TODO: handle in source
            }

            source_raw.push_str(contents.as_str());

            let source = CString::new(source_raw);
            if source.is_err() {
                panic!("Unable to create CString from file");
            }
            let shader = Self {
                id: gl::CreateShader(shader_type),
            };

            gl::ShaderSource(
                shader.id,
                1,
                &std::ffi::CStr::as_ptr(&source.unwrap()),
                std::ptr::null(),
            );
            gl::CompileShader(shader.id);
            let mut success = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as GLint {
                let mut error_log_size = 0;
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = vec![0u8; error_log_size as usize];
                gl::GetShaderInfoLog(
                    shader.id,
                    error_log_size,
                    std::ptr::null_mut(),
                    error_log.as_mut_ptr() as *mut GLchar,
                );
                error_log.set_len(error_log_size as usize);
                let log = String::from_utf8(error_log).unwrap();
                println!("Error compiling {}: {:?}", path_str, log);
            }
            Some(shader)
        }
    }
}
