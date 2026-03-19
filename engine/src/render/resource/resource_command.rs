use flume::Sender;
use include_assets::NamedArchive;

use crate::{
    render::{
        mesh::Mesh,
        resource::{
            shader::{shader_configuration::ShaderConfiguration, shader_program::ShaderProgram},
            texture::texture::Texture,
        },
    },
    shared::engine_error::EngineError,
};

pub enum ResourceCommand {
    AttachArchive {
        archive: NamedArchive,
    },
    LoadStaticTexture {
        texture_name: String,
        response_tx: Sender<Result<Texture, EngineError>>,
    },
    LoadAnimatedTexture {
        texture_names: Vec<String>,
        animation_time: u128,
        response_tx: Sender<Result<Texture, EngineError>>,
    },
    LoadShader {
        shader_configuration: ShaderConfiguration,
        response_tx: Sender<Result<ShaderProgram, EngineError>>,
    },
    LoadMesh {
        vertices: Vec<f32>,
        indices: Vec<u32>,
        uvs: Vec<f32>,
        response_tx: Sender<Result<Mesh, EngineError>>,
    },
    UnloadMesh {
        mesh: Mesh,
    },
}
