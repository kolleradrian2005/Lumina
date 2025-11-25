use std::sync::Arc;

use crate::{model::mesh::Mesh};

#[derive(Debug)]
pub struct DropMeshRequest {
    pub mesh: Arc<Mesh>,
}
