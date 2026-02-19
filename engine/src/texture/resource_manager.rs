use std::{collections::HashMap, sync::Arc};

use flume::Sender;
use include_assets::NamedArchive;

use crate::{
    math::vec3::Vec3,
    model::{mesh::Mesh, model::Model, sprite},
    shader::{
        parameter_schema::ParameterSchema, shader_configuration::ShaderConfiguration,
        shader_parameter_type::ShaderParameterType, shader_program::ShaderProgram,
    },
    texture::{resource_command::ResourceCommand, texture::StaticTexture},
};

use super::{
    resource_provider::ResourceProvider,
    texture::{StaticColor, Texture},
};

pub struct ResourceManager {
    place_holder_model: Model,
    //place_holder_font: FontTexture,
    models: HashMap<String, Model>,
    //fonts: HashMap<String, FontTexture>,
    shader_programs: HashMap<String, Arc<ShaderProgram>>,
    loader_tx: Sender<ResourceCommand>,
}

impl ResourceManager {
    pub fn new(loader_tx: Sender<ResourceCommand>) -> Self {
        let (vertices, indices, uvs) = sprite::square(1.0);
        let (tx, rx) = flume::bounded(1);
        loader_tx
            .send(ResourceCommand::LoadMesh {
                vertices,
                indices,
                uvs,
                response_tx: tx,
            })
            .expect("Render thread died!");
        let default_mesh = rx
            .recv()
            .expect("Render thread died!")
            .expect("Failed to load default mesh");
        ResourceManager {
            place_holder_model: Model::new(default_mesh),
            //place_holder_font: FontTexture::new(),
            models: HashMap::new(),
            //fonts: HashMap::new(),
            loader_tx,
            shader_programs: HashMap::new(),
        }
    }

    fn send_resource_command(&self, command: ResourceCommand) {
        self.loader_tx.send(command).expect("Render thread died!")
    }
}

impl ResourceProvider for ResourceManager {
    fn load_default_models(&mut self) {
        let (vertices, indices, uvs) = sprite::square(1.0);
        let square_mesh = self
            .load_mesh(vertices, indices, uvs)
            .expect("Could not load default model");
        let mut square = Model::new(square_mesh);
        square.set_texture(StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into());
        self.save_model("square", square);
    }

    fn load_default_shaders(&mut self) {
        let mut model_shader_parameter_schema = ParameterSchema {
            required_params: vec![
                ("uModelMatrix".to_string(), ShaderParameterType::Mat4),
                ("uObjectType".to_string(), ShaderParameterType::Int),
                ("uTextureType".to_string(), ShaderParameterType::Int),
                ("uColor".to_string(), ShaderParameterType::Vec3),
                ("uFlipped".to_string(), ShaderParameterType::Bool),
                ("uTerrainIsUphill".to_string(), ShaderParameterType::Bool),
                ("uTerrainHeight".to_string(), ShaderParameterType::Float),
            ],
        };
        let model_shader_configuration = ShaderConfiguration {
            fragment_shader_name: "model.frag".into(),
            vertex_shader_name: "model.vert".into(),
            tess_evaluation_shader_name: None,
            tess_control_shader_name: None,
            parameter_schema: model_shader_parameter_schema.clone(),
        };
        // Tesselation shaders are not supported on gles 3, which is used on android, so only include them for other platforms
        let shader_with_tesselation_configuration = cfg!(not(target_os = "android")).then(|| {
            model_shader_parameter_schema
                .required_params
                .push(("uCurrent".to_string(), ShaderParameterType::Float));
            ShaderConfiguration {
                fragment_shader_name: "model.frag".into(),
                vertex_shader_name: "model.vert".into(),
                tess_evaluation_shader_name: Some("model.tese".into()),
                tess_control_shader_name: Some("model.tesc".into()),
                parameter_schema: model_shader_parameter_schema,
            }
        });

        self.load_shader("model", model_shader_configuration.clone())
            .expect("Failed to load model shader");

        self.load_shader(
            "model_with_tesselation",
            shader_with_tesselation_configuration.unwrap_or(model_shader_configuration),
        )
        .expect("Failed to load model_with_tesselation shader");
    }

    /*fn load_fonts(&mut self) {
            if let Some(default_font) = self.load_font("Raleway-Regular.ttf") {
                self.save_font("default", default_font);
            }
        }
    */

    fn save_model(&mut self, name: &str, model: Model) {
        self.models.insert(name.to_string(), model);
    }

    fn get_model(&self, name: &str) -> Model {
        match self.models.get(name) {
            Some(model) => model,
            None => &self.place_holder_model,
        }
        .clone()
    }

    /*
        fn get_font(&self, name: &str) -> FontTexture {
            if let Some(font) = self.fonts.get(name) {
                font.clone()
            } else {
                self.place_holder_font.clone()
            }
        }
    */

    fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture> {
        let (tx, rx) = flume::bounded(1);
        self.send_resource_command(ResourceCommand::LoadStaticTexture {
            texture_name: texture_name.to_string(),
            response_tx: tx,
        });

        match rx.recv() {
            Ok(texture) => texture,
            Err(_) => {
                println!("Failed to load static texture: {:?}", texture_name);
                None
            }
        }
    }

    fn load_animated_texture(
        &mut self,
        texture_names: &[&str],
        animation_time: u128,
    ) -> Option<Texture> {
        let (tx, rx) = flume::bounded(1);
        self.send_resource_command(ResourceCommand::LoadAnimatedTexture {
            texture_names: texture_names.iter().map(|s| s.to_string()).collect(),
            animation_time,
            response_tx: tx,
        });

        match rx.recv() {
            Ok(texture) => texture,
            Err(_) => {
                println!("Failed to load animated texture: {:?}", texture_names);
                None
            }
        }
    }

    fn attach_archive(&mut self, archive: NamedArchive) {
        self.loader_tx
            .send(ResourceCommand::AttachArchive { archive })
            .expect("Render thread died!");
    }

    fn get_shader(&self, shader_name: &str) -> Arc<ShaderProgram> {
        self.shader_programs
            .get(shader_name)
            .expect(&format!("Shader {} not loaded!", shader_name))
            .clone()
    }

    fn load_shader(
        &mut self,
        shader_name: &str,
        shader_configuration: ShaderConfiguration,
    ) -> Option<Arc<ShaderProgram>> {
        let (tx, rx) = flume::bounded(1);
        self.send_resource_command(ResourceCommand::LoadShader {
            shader_configuration,
            response_tx: tx,
        });
        let shader_program = match rx.recv() {
            Ok(shader) => shader,
            Err(_) => {
                println!("Failed to load shader: {:?}", shader_name);
                None
            }
        };
        if let Some(shader) = shader_program {
            let shader = Arc::new(shader);
            self.shader_programs
                .insert(shader_name.to_string(), shader.clone());
            return Some(shader);
        }
        None
    }
}

impl ResourceManager {
    /*fn save_font(&mut self, name: &str, font: FontTexture) {
            self.fonts.insert(name.to_string(), font);
        }

        fn load_font(&mut self, font_name: &str) -> Option<FontTexture> {
            let (tx, rx) = flume::bounded(1);
            self.send_resource_command(ResourceCommand::LoadFont {
                    font_name: font_name.to_string(),
                    response_tx: tx,
                });
            match rx.recv() {
                Ok(texture) => texture,
                Err(_) => {
                    println!("Failed to load font: {:?}", font_name);
                    None
                }
            }
        }
    */
    pub fn load_mesh(
        &mut self,
        vertices: Vec<f32>,
        indices: Vec<u32>,
        uvs: Vec<f32>,
    ) -> Option<Mesh> {
        let (tx, rx) = flume::bounded(1);
        self.send_resource_command(ResourceCommand::LoadMesh {
            vertices,
            indices,
            uvs,
            response_tx: tx,
        });

        match rx.recv() {
            Ok(mesh) => mesh,
            Err(_) => {
                println!("Failed to load mesh");
                None
            }
        }
    }

    pub fn unload_mesh(&mut self, mesh: Mesh) {
        self.send_resource_command(ResourceCommand::UnloadMesh { mesh });
    }

    pub fn load_mesh_from_texture(&mut self, texture: &StaticTexture) -> Option<Mesh> {
        let (width, height) = texture.get_normalized_dimensions();
        let (vertices, indices, uvs) = sprite::rectangle(width, height);
        self.load_mesh(vertices, indices, uvs)
    }
}
