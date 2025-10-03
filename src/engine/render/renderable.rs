use std::sync::Arc;

use crate::engine::{
    model::mesh::Mesh, scene::world::component::shader_params_component::ShaderParamsComponent,
    texture::texture::Texture,
};

use super::scene_renderer::ObjectType;

#[derive(Clone, Debug)]
pub struct Renderable {
    pub mesh: Option<Arc<Mesh>>,
    pub texture: Texture,
    pub is_flipped: bool,
    pub transform_matrix: [[f32; 4]; 4],
    pub object_type: ObjectType,
    pub shader_params: Option<ShaderParamsComponent>,
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
