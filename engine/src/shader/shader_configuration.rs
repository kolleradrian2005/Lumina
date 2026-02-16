use crate::shader::parameter_schema::ParameterSchema;

#[derive(Clone)]
pub struct ShaderConfiguration {
    pub fragment_shader_name: String,
    pub vertex_shader_name: String,
    pub tess_evaluation_shader_name: Option<String>,
    pub tess_control_shader_name: Option<String>,
    pub parameter_schema: ParameterSchema,
}

impl ShaderConfiguration {
    pub fn new(
        fragment_shader: &str,
        vertex_shader: &str,
        parameter_schema: ParameterSchema,
    ) -> Self {
        Self {
            fragment_shader_name: fragment_shader.to_string(),
            vertex_shader_name: vertex_shader.to_string(),
            tess_evaluation_shader_name: None,
            tess_control_shader_name: None,
            parameter_schema,
        }
    }
}
