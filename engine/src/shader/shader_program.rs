use gl::types::GLuint;
use include_assets::NamedArchive;

use crate::shader::{
    parameter_schema::ParameterSchema, shader::Shader, shader_configuration::ShaderConfiguration,
    shader_handler,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShaderHandle {
    pub id: GLuint,
    pub has_tesselation: bool,
}

pub struct ShaderProgram {
    handle: ShaderHandle,
    //uniform_locations: HashMap<String, GLint>,
    parameter_schema: ParameterSchema,
}

impl ShaderProgram {
    pub fn load_from_configuration(
        archive: &NamedArchive,
        shader_configuration: ShaderConfiguration,
    ) -> Option<Self> {
        unsafe {
            // TODO: Handle shader loading failure inbetween shaders, currently if one shader fails to load, the rest will still be loaded and compiled, which is a waste of resources
            let fragment_shader = Shader::new(
                archive,
                &shader_configuration.fragment_shader_name,
                gl::FRAGMENT_SHADER,
            )?;
            let vertex_shader = Shader::new(
                archive,
                &shader_configuration.vertex_shader_name,
                gl::VERTEX_SHADER,
            )?;
            let mut has_tesselation = false;
            let mut shaders = vec![fragment_shader, vertex_shader];
            if let Some(tesselation_control_shader_name) =
                shader_configuration.tess_control_shader_name
            {
                let tesc_shader = Shader::new(
                    archive,
                    &tesselation_control_shader_name,
                    gl::TESS_CONTROL_SHADER,
                )?;
                shaders.push(tesc_shader);
                has_tesselation = true;
            }
            if let Some(tesselation_shader_name) = shader_configuration.tess_evaluation_shader_name
            {
                let tese_shader = Shader::new(
                    archive,
                    &tesselation_shader_name,
                    gl::TESS_EVALUATION_SHADER,
                )?;
                shaders.push(tese_shader);
                has_tesselation = true;
            }
            let id = shader_handler::load_program(&shaders);
            /*let mut uniform_locations = HashMap::new();
            for (parameter_name, _) in &shader_configuration.parameter_schema.required_params {
                let uniform_location = gl::GetUniformLocation(
                    id,
                    std::ffi::CStr::as_ptr(&CString::new(parameter_name.clone()).unwrap()),
                );
                uniform_locations.insert(parameter_name.clone(), uniform_location);
                if uniform_location == -1 {
                    eprintln!(
                        "Warning: Uniform '{}' not found in shader program '{}'",
                        parameter_name, parameter_name
                    );
                }
            }*/

            let shader_program = Self {
                handle: ShaderHandle {
                    id,
                    has_tesselation,
                },
                //uniform_locations,
                parameter_schema: shader_configuration.parameter_schema,
            };

            shader_handler::bind_attributes_to_program(&shader_program, 0, "position");
            shader_handler::bind_attributes_to_program(&shader_program, 1, "uv");
            Some(shader_program)
        }
    }

    pub fn get_handle(&self) -> ShaderHandle {
        self.handle
    }

    pub fn get_parameter_schema(&self) -> &ParameterSchema {
        &self.parameter_schema
    }
}
