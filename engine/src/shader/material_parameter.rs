use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    shader::shader_parameter_type::ShaderParameterType,
};

#[derive(Debug, Clone)]
pub enum MaterialParameter {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Mat4([[f32; 4]; 4]),
    Int(i32),
    Bool(bool),
    Vec2Array(Vec<Vec2>),
}

impl MaterialParameter {
    pub const fn default_value_for(
        shader_parameter_type: &ShaderParameterType,
    ) -> MaterialParameter {
        match shader_parameter_type {
            ShaderParameterType::Float => MaterialParameter::Float(0.0),
            ShaderParameterType::Vec2 => MaterialParameter::Vec2(Vec2::new(0.0, 0.0)),
            ShaderParameterType::Vec3 => MaterialParameter::Vec3(Vec3::new(0.0, 0.0, 0.0)),
            ShaderParameterType::Mat4 => MaterialParameter::Mat4([[0.0; 4]; 4]),
            ShaderParameterType::Int => MaterialParameter::Int(0),
            ShaderParameterType::Bool => MaterialParameter::Bool(false),
            ShaderParameterType::Vec2Array => MaterialParameter::Vec2Array(vec![]),
        }
    }
}

impl From<f32> for MaterialParameter {
    fn from(v: f32) -> Self {
        MaterialParameter::Float(v)
    }
}

impl From<Vec3> for MaterialParameter {
    fn from(v: Vec3) -> Self {
        MaterialParameter::Vec3(v)
    }
}

impl From<[[f32; 4]; 4]> for MaterialParameter {
    fn from(v: [[f32; 4]; 4]) -> Self {
        MaterialParameter::Mat4(v)
    }
}

impl From<i32> for MaterialParameter {
    fn from(v: i32) -> Self {
        MaterialParameter::Int(v)
    }
}

impl From<bool> for MaterialParameter {
    fn from(v: bool) -> Self {
        MaterialParameter::Bool(v)
    }
}
