use std::sync::Arc;

use crate::engine::{
    model::mesh::Mesh,
    render::{renderable::MeshLoadState, scene_renderer::ObjectType},
};

use super::component::Component;

#[derive(Clone)]
pub struct ModelComponent {
    pub mesh: MeshLoadState,
    pub object_type: ObjectType,
}

impl ModelComponent {
    pub fn new(mesh: MeshLoadState, object_type: Option<ObjectType>) -> Self {
        Self {
            mesh,
            object_type: object_type.unwrap_or(ObjectType::Default),
        }
    }
}

impl From<Arc<Mesh>> for ModelComponent {
    fn from(mesh: Arc<Mesh>) -> Self {
        Self {
            mesh: MeshLoadState::Loaded(mesh),
            object_type: ObjectType::Default,
        }
    }
}

impl Component for ModelComponent {}
