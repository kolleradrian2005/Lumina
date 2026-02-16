use flume::Sender;
use include_assets::NamedArchive;

use crate::{
    model::mesh::Mesh,
    shader::{shader_configuration::ShaderConfiguration, shader_program::ShaderProgram},
    texture::texture::Texture,
};

pub enum ResourceCommand {
    AttachArchive {
        archive: NamedArchive,
    },
    /*LoadFont {
        font_name: String,
        response_tx: Sender<Option<FontTexture>>,
    },*/
    LoadStaticTexture {
        texture_name: String,
        response_tx: Sender<Option<Texture>>,
    },
    LoadAnimatedTexture {
        texture_names: Vec<String>,
        animation_time: u128,
        response_tx: Sender<Option<Texture>>,
    },
    LoadShader {
        shader_configuration: ShaderConfiguration,
        response_tx: Sender<Option<ShaderProgram>>,
    },
    LoadMesh {
        vertices: Vec<f32>,
        indices: Vec<u32>,
        uvs: Vec<f32>,
        response_tx: Sender<Option<Mesh>>,
    },
    UnloadMesh {
        mesh: Mesh,
    },
}
