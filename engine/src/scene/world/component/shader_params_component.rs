use crate::scene::world::component::component::Component;

#[derive(Clone, Debug)]
pub struct ShaderParamsComponent {
    pub params: Box<[ShaderParam]>,
}

impl Component for ShaderParamsComponent {}

#[derive(Clone, Debug)]
pub enum ShaderParam {
    IsUphill(bool),
    Height(f32),
    Current(f32),
}
