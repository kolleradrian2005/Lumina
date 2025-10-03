use std::sync::Arc;

use crate::engine::{model::mesh::Mesh};

#[derive(Debug)]
pub struct DropMeshRequest {
    pub mesh: Arc<Mesh>,
}
