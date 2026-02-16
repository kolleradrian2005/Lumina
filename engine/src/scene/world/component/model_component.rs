use std::sync::Arc;

use crate::model::mesh::Mesh;

use super::component::Component;

#[derive(Component, Clone)]
pub struct ModelComponent {
    pub mesh: Arc<Mesh>,
    //pub object_type: ObjectType,
}

impl ModelComponent {
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh: mesh.into() }
    }
}

impl From<Arc<Mesh>> for ModelComponent {
    fn from(mesh: Arc<Mesh>) -> Self {
        Self { mesh }
    }
}
