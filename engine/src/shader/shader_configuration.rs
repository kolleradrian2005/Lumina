use crate::shader::{
    parameter_schema::ParameterSchema, shader_parameter_type::ShaderParameterType,
};

#[derive(Clone, Debug)]
pub struct UniformBufferBinding {
    pub binding_index: u32,
    pub fields: Vec<(String, ShaderParameterType)>,
}

#[derive(Clone, Debug)]
pub struct ShaderConfiguration {
    pub fragment_shader_name: String,
    pub vertex_shader_name: String,
    pub tess_evaluation_shader_name: Option<String>,
    pub tess_control_shader_name: Option<String>,
    pub parameter_schema: ParameterSchema,
    //pub uniform_buffers: Vec<UniformBufferBinding>,
}
