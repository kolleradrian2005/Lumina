use std::sync::Arc;

use crate::render::mesh::Mesh;

use super::component::Component;

#[derive(Component, Clone)]
pub struct Model {
    pub mesh: Arc<Mesh>,
}

impl Model {
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh: mesh.into() }
    }
}

impl From<Arc<Mesh>> for Model {
    fn from(mesh: Arc<Mesh>) -> Self {
        Self { mesh }
    }
}
