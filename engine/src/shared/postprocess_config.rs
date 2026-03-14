use crate::logic::scene::ecs::component::material_component::MaterialComponent;

#[derive(Clone, Debug)]
pub struct PostprocessConfig {
    pub material: MaterialComponent,
}
