use std::collections::HashMap;

use include_assets::NamedArchive;

use crate::{
    math::vec3::Vec3,
    model::{model::Model, sprite},
};

use super::{
    font_texture::FontTexture,
    resource_provider::ResourceProvider,
    texture::{StaticColor, Texture},
    texture_handler::TextureHandler,
};

pub struct ResourceManager {
    place_holder_model: Model,
    place_holder_font: FontTexture,
    models: HashMap<String, Model>,
    fonts: HashMap<String, FontTexture>,
    texture_handler: TextureHandler,
    archives: Vec<NamedArchive>,
}

impl ResourceManager {
    pub fn new(archive: NamedArchive) -> Self {
        ResourceManager {
            place_holder_model: sprite::square(1.0),
            place_holder_font: FontTexture::new(),
            models: HashMap::new(),
            fonts: HashMap::new(),
            texture_handler: TextureHandler::new(),
            archives: vec![archive],
        }
    }
}

impl ResourceProvider for ResourceManager {
    fn load_default_models(&mut self) {
        let mut square = sprite::square(1.0);
        square.set_texture(StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into());
        self.save_model("square", square);
    }

    fn load_fonts(&mut self) {
        for archive in self.archives.iter().rev() {
            if let Some(font) = self
                .texture_handler
                .load_font(archive, "Raleway-Regular.ttf")
            {
                self.save_font("default", font);
                return;
            }
        }
    }

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

    fn get_font(&self, name: &str) -> FontTexture {
        if let Some(font) = self.fonts.get(name) {
            font.clone()
        } else {
            self.place_holder_font.clone()
        }
    }

    fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture> {
        for archive in self.archives.iter().rev() {
            if let Some(texture) = self
                .texture_handler
                .load_static_texture(archive, texture_name)
            {
                return Some(texture);
            }
        }
        println!("Failed to load static texture: {:?}", texture_name);
        None
    }

    fn load_animated_texture(
        &mut self,
        texture_names: &[&str],
        animation_time: u128,
    ) -> Option<Texture> {
        for archive in self.archives.iter().rev() {
            if let Some(texture) =
                self.texture_handler
                    .load_animated_texture(archive, texture_names, animation_time)
            {
                return Some(texture);
            }
        }
        println!("Failed to load animated texture: {:?}", texture_names);
        None
    }
    fn attach_archive(&mut self, archive: NamedArchive) {
        self.archives.push(archive);
    }
}

impl ResourceManager {
    fn save_font(&mut self, name: &str, font: FontTexture) {
        self.fonts.insert(name.to_string(), font);
    }
}
