use crate::scene::world::component::component::Component;

#[derive(Component, Clone, Debug)]
pub struct ShaderParamsComponent {
    pub params: Box<[ShaderParam]>,
}

#[derive(Clone, Debug)]
pub enum ShaderParam {
    IsUphill(bool),
    Height(f32),
    Current(f32),
}
