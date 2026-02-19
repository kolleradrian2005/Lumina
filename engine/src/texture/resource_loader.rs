use flume::Receiver;
use include_assets::NamedArchive;

use crate::{
    model::mesh::Mesh,
    shader::shader_program::ShaderProgram,
    texture::{
        resource_command::ResourceCommand, texture::Texture, texture_handler::TextureHandler,
    },
};

pub struct ResourceLoader {
    loader_rx: Receiver<ResourceCommand>,
    texture_handler: TextureHandler,
    archives: Vec<NamedArchive>,
}

impl ResourceLoader {
    pub fn new(loader_rx: Receiver<ResourceCommand>) -> Self {
        ResourceLoader {
            loader_rx,
            texture_handler: TextureHandler::new(),
            archives: Vec::new(),
        }
    }
    pub fn run(&mut self) {
        for command in self.loader_rx.try_iter() {
            match command {
                ResourceCommand::LoadStaticTexture {
                    texture_name,
                    response_tx,
                } => {
                    let mut texture: Option<Texture> = None;
                    for archive in self.archives.iter().rev() {
                        if let Some(tex) = self
                            .texture_handler
                            .load_static_texture(archive, &texture_name)
                        {
                            texture = Some(tex);
                            break;
                        }
                    }
                    let _ = response_tx.send(texture);
                }
                ResourceCommand::LoadAnimatedTexture {
                    texture_names,
                    animation_time,
                    response_tx,
                } => {
                    let mut texture: Option<Texture> = None;
                    for archive in self.archives.iter().rev() {
                        if let Some(tex) = self.texture_handler.load_animated_texture(
                            archive,
                            &texture_names,
                            animation_time,
                        ) {
                            texture = Some(tex);
                            break;
                        }
                    }
                    let _ = response_tx.send(texture);
                }
                ResourceCommand::AttachArchive { archive } => {
                    self.archives.push(archive);
                }
                ResourceCommand::LoadMesh {
                    vertices,
                    indices,
                    uvs,
                    response_tx,
                } => {
                    let mesh = Mesh::new(&vertices, &indices, &uvs);
                    // Upon error, None may be returned
                    let _ = response_tx.send(mesh.into());
                }
                ResourceCommand::UnloadMesh { mesh } => mesh.destroy(),
                ResourceCommand::LoadShader {
                    shader_configuration,
                    response_tx,
                } => {
                    let mut shader_program: Option<ShaderProgram> = None;
                    for archive in self.archives.iter().rev() {
                        if let Some(shad) = ShaderProgram::load_from_configuration(
                            archive,
                            shader_configuration.clone(),
                        ) {
                            shader_program = Some(shad);
                            break;
                        }
                    }
                    let _ = response_tx.send(shader_program);
                }
            }
        }
    }
}
