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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DrawMode {
    Triangles,
    Lines,
    Patches,
}

#[derive(Component, Clone, Debug)]
pub struct MaterialComponent {
    pub texture: Texture,
    pub shader: ShaderHandle,
    pub parameters: HashMap<String, MaterialParameter>,
    pub draw_mode: DrawMode,
    //pub uniform_buffers: HashMap<u32, Vec<u8>>,
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
            draw_mode: DrawMode::Triangles,
        }
    }

    pub fn with_draw_mode(mut self, draw_mode: DrawMode) -> Self {
        self.draw_mode = draw_mode;
        self
    }

    pub fn with_param<P: Into<MaterialParameter>>(mut self, name: &str, value: P) -> Self {
        let param = value.into();
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
