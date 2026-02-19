use std::sync::Arc;

use crate::{model::mesh::Mesh, scene::world::component::material_component::MaterialComponent};

#[derive(Clone, Debug)]
pub struct RenderEntity {
    pub mesh: Arc<Mesh>,
    pub material: MaterialComponent,
    pub is_flipped: bool,
    pub transform_matrix: [[f32; 4]; 4], // TODO: remove it and add it to the material component
                                         //pub object_type: ObjectType,
                                         //pub shader_params: Option<ShaderParamsComponent>,
}

#[derive(Clone, Debug)]
pub enum MeshLoadState {
    CreateRequest {
        vertices: Box<[f32]>,
        indices: Box<[u32]>,
        uvs: Box<[f32]>,
    },
    PendingRequest {
        id: u64,
    },
    Loaded(Arc<Mesh>),
}
