use crate::render::resource::shader::shader_parameter_type::ShaderParameterType;

#[derive(Debug, Clone)]
pub struct ParameterSchema {
    pub required_params: Vec<(String, ShaderParameterType)>,
}
