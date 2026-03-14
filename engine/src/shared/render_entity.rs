use std::sync::Arc;

use crate::{
    logic::scene::ecs::component::material_component::MaterialComponent, render::mesh::Mesh,
};

#[derive(Clone, Debug)]
pub struct RenderEntity {
    pub mesh: Arc<Mesh>,
    pub material: MaterialComponent,
    pub z_index: f32,
}
