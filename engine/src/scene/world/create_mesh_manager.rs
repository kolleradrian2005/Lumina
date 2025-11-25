use std::{collections::HashMap};

use crate::{render::renderable::MeshLoadState};

pub struct CreateMeshManager {
    next_id: u64,
    meshes: HashMap<u64, MeshLoadState>,
}

impl CreateMeshManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            meshes: HashMap::new(),
        }
    }

    pub fn request_mesh(&mut self, create_request: MeshLoadState) -> MeshLoadState {
        let id = self.next_id;
        self.next_id += 1;
        self.meshes.insert(id, create_request);
        MeshLoadState::PendingRequest { id }
    }

    pub fn take_mesh(&mut self, id: u64) -> Option<MeshLoadState> {
        self.meshes.remove(&id)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u64, &mut MeshLoadState)> {
        self.meshes.iter_mut()
    }
}
