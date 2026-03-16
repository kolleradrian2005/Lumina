use std::sync::Arc;

use crate::{logic::ecs::component::material::Material, render::mesh::Mesh};

#[derive(Clone, Debug)]
pub struct RenderEntity {
    pub mesh: Arc<Mesh>,
    pub material: Material,
    pub z_index: f32,
}
