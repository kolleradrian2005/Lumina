use gl::types::*;
use include_assets::NamedArchive;
use std::{
    collections::HashMap,
    ffi::CString,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    engine_config,
    render::resource::shader::{
        shader_configuration::ShaderConfiguration,
        shader_handle::ShaderHandle,
        shader_handler,
        shader_program::{ShaderProgram, ShaderProgramHandle},
    },
    shared::engine_error::EngineError,
};

pub struct ShaderLoader {
    id_map: HashMap<PathBuf, ShaderHandle>,
}

impl ShaderLoader {
    pub fn new() -> Self {
        ShaderLoader {
            id_map: HashMap::new(),
        }
    }

    fn load_shader(
        &mut self,
        archive: &NamedArchive,
        shader_name: &str,
        shader_type: GLenum,
    ) -> Result<ShaderHandle, EngineError> {
        unsafe {
            let path = Path::new(engine_config::SHADERS_PATH).join(shader_name.replace("/", "\\"));
            let binding = path.to_string_lossy().replace("/", "\\");
            let path_str = binding.as_str();

            let asset = archive.get(path_str);
            if asset.is_none() {
                return Err(EngineError::FileNotFound(path_str.to_string()));
            }
            if let Some(texture) = self.id_map.get(&path) {
                return Ok(texture.clone());
            }
            let mut contents = String::new();

            if let Err(err) = &asset.unwrap().read_to_string(&mut contents) {
                return Err(EngineError::Generic(format!(
                    "Failed to read shader file '{}': {}",
                    path_str, err
                )));
            }

            let mut source_raw = String::from(engine_config::SHADER_VERSION_HEADER);

            if cfg!(target_os = "android") {
                source_raw.push_str("#define ES\r\n"); // TODO: handle in source
            }

            source_raw.push_str(contents.as_str());

            let source = CString::new(source_raw);
            if source.is_err() {
                return Err(EngineError::Generic(format!(
                    "Failed to create CString from shader file '{}': {}",
                    path_str,
                    source.err().unwrap()
                )));
            }
            let shader = ShaderHandle {
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
                return Err(EngineError::ShaderCompilation(path_str.to_string(), log));
            }
            self.id_map.insert(path, shader.clone());
            Ok(shader)
        }
    }

    pub fn load_shader_program(
        &mut self,
        archive: &NamedArchive,
        shader_configuration: ShaderConfiguration,
    ) -> Result<ShaderProgram, EngineError> {
        unsafe {
            // TODO: Handle shader loading failure inbetween shaders, currently if one shader fails to load, the rest will still be loaded and compiled, which is a waste of resources
            let fragment_shader = self.load_shader(
                archive,
                &shader_configuration.fragment_shader_name,
                gl::FRAGMENT_SHADER,
            )?;
            let vertex_shader = self.load_shader(
                archive,
                &shader_configuration.vertex_shader_name,
                gl::VERTEX_SHADER,
            )?;
            let mut has_tesselation = false;
            let mut shaders = vec![fragment_shader, vertex_shader];
            if let Some(tesselation_control_shader_name) =
                shader_configuration.tess_control_shader_name
            {
                let tesc_shader = self.load_shader(
                    archive,
                    &tesselation_control_shader_name,
                    gl::TESS_CONTROL_SHADER,
                )?;
                shaders.push(tesc_shader);
                has_tesselation = true;
            }
            if let Some(tesselation_shader_name) = shader_configuration.tess_evaluation_shader_name
            {
                let tese_shader = self.load_shader(
                    archive,
                    &tesselation_shader_name,
                    gl::TESS_EVALUATION_SHADER,
                )?;
                shaders.push(tese_shader);
                has_tesselation = true;
            }
            let id = shader_handler::load_program(&shaders);

            let shader_program = ShaderProgram::new(
                ShaderProgramHandle {
                    id,
                    has_tesselation,
                },
                shader_configuration.parameter_schema,
            );

            shader_handler::bind_attributes_to_program(&shader_program, 0, "position");
            shader_handler::bind_attributes_to_program(&shader_program, 1, "uv");
            Ok(shader_program)
        }
    }
}
