use crate::logic::ecs::component::material::Material;

#[derive(Clone, Debug)]
pub struct PostprocessConfig {
    pub material: Material,
}
