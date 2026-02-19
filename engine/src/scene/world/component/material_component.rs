use std::{collections::HashMap, sync::Arc};

use crate::{
    shader::{
        material_parameter::MaterialParameter,
        shader_program::{ShaderHandle, ShaderProgram},
    },
    texture::texture::Texture,
};

use super::component::Component;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ObjectType {
    Default,
    Terrain,
    SeaGrass,
}

#[derive(Component, Clone, Debug)]
pub struct MaterialComponent {
    pub texture: Texture,
    pub shader: ShaderHandle,
    pub parameters: HashMap<String, MaterialParameter>,
}

impl MaterialComponent {
    pub fn new(texture: Texture, shader: Arc<ShaderProgram>) -> Self {
        let mut parameters = HashMap::new();
        for (param_name, param_type) in &shader.get_parameter_schema().required_params {
            parameters.insert(
                param_name.clone(),
                MaterialParameter::default_value_for(param_type),
            );
        }
        Self {
            texture,
            shader: shader.get_handle(),
            parameters,
        }
    }

    pub fn with_param<P: Into<MaterialParameter>>(mut self, name: &str, value: P) -> Self {
        let param = value.into();
        /*let schema = self.shader.get_parameter_schema();
        if let Some((_, expected_type)) = schema.required_params.iter().find(|(n, _)| n == name) {
            {
                let actual_type = Self::type_of(&param);
                assert_eq!(
                    actual_type, *expected_type,
                    "Parameter {} type mismatch: expected {:?}, got {:?}",
                    name, expected_type, actual_type
                );
            }
            self.parameters.insert(name.to_string(), param);
        } else {
            panic!("Parameter {} not defined in shader", name);
        }*/
        self.parameters.insert(name.to_string(), param);
        self
    }

    pub fn set_param<P: Into<MaterialParameter>>(&mut self, name: &str, value: P) {
        let param = value.into();
        self.parameters.insert(name.to_string(), param);
    }

    pub fn get_param(&self, name: &str) -> Option<&MaterialParameter> {
        self.parameters.get(name)
    }
}
