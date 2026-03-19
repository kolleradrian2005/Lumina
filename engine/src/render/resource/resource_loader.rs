use flume::Receiver;
use include_assets::NamedArchive;

use crate::{
    render::{
        mesh::Mesh,
        resource::{
            resource_command::ResourceCommand,
            shader::{shader_loader::ShaderLoader, shader_program::ShaderProgram},
            texture::{texture::Texture, texture_loader::TextureLoader},
        },
    },
    shared::engine_error::EngineError,
};

pub struct ResourceLoader {
    loader_rx: Receiver<ResourceCommand>,
    texture_loader: TextureLoader,
    shader_loader: ShaderLoader,
    archives: Vec<NamedArchive>,
}

impl ResourceLoader {
    pub fn new(loader_rx: Receiver<ResourceCommand>) -> Self {
        ResourceLoader {
            loader_rx,
            texture_loader: TextureLoader::new(),
            shader_loader: ShaderLoader::new(),
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
                    let mut texture: Result<Texture, EngineError> =
                        Err(EngineError::Generic("No archives available".to_string()));
                    for archive in self.archives.iter().rev() {
                        match self
                            .texture_loader
                            .load_static_texture(archive, &texture_name)
                        {
                            Err(EngineError::FileNotFound(t)) => {
                                texture = Err(EngineError::FileNotFound(t)); // Continue searching other archives
                            }
                            result => {
                                texture = result;
                                break;
                            }
                        }
                    }
                    let _ = response_tx.send(texture);
                }
                ResourceCommand::LoadAnimatedTexture {
                    texture_names,
                    animation_time,
                    response_tx,
                } => {
                    let mut texture: Result<Texture, EngineError> =
                        Err(EngineError::Generic("No archives available".to_string()));
                    for archive in self.archives.iter().rev() {
                        match self.texture_loader.load_animated_texture(
                            archive,
                            &texture_names,
                            animation_time,
                        ) {
                            Err(EngineError::FileNotFound(t)) => {
                                texture = Err(EngineError::FileNotFound(t)); // Continue searching other archives
                            }
                            result => {
                                texture = result;
                                break;
                            }
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
                    // Upon error, Err may be returned
                    let _ = response_tx.send(Ok(mesh));
                }
                ResourceCommand::UnloadMesh { mesh } => mesh.destroy(),
                ResourceCommand::LoadShader {
                    shader_configuration,
                    response_tx,
                } => {
                    let mut shader_program: Result<ShaderProgram, EngineError> =
                        Err(EngineError::Generic("No archives available".to_string()));
                    for archive in self.archives.iter().rev() {
                        match self
                            .shader_loader
                            .load_shader_program(archive, shader_configuration.clone())
                        {
                            Err(EngineError::FileNotFound(t)) => {
                                shader_program = Err(EngineError::FileNotFound(t));
                                // Continue searching other archives
                            }
                            result => {
                                shader_program = result;
                                break;
                            }
                        }
                    }
                    let _ = response_tx.send(shader_program);
                }
            }
        }
    }
}
