use std::collections::HashMap;

use crate::engine::{
    math::{vec2::Vec2, vec3::Vec3},
    model::{model::Model, sprite},
    transformable::Transformable,
};

use super::{
    font_texture::FontTexture,
    texture::{StaticColor, Texture},
    texture_handler::TextureHandler,
};

pub struct ResourceManager {
    place_holder_model: Model,
    place_holder_font: FontTexture,
    models: HashMap<String, Model>,
    fonts: HashMap<String, FontTexture>,
    texture_handler: TextureHandler,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            place_holder_model: sprite::square(1.0),
            place_holder_font: FontTexture::new(),
            models: HashMap::new(),
            fonts: HashMap::new(),
            texture_handler: TextureHandler::new(),
        }
    }

    pub fn preload_models(&mut self) {
        let mut square = sprite::square(1.0);
        square.set_texture(StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into());
        let mut bubble = square.clone();
        bubble.set_scale(Vec2::uniform(0.01));
        if let Some(texture) = self.texture_handler.load_static_texture("bubble.png") {
            bubble.set_texture(texture);
        }
        if let Some(Texture::StaticTexture(texture)) =
            self.texture_handler.load_static_texture("seagrass0.png")
        {
            let mut seagrass = sprite::from_texture(texture);
            seagrass.set_scale(Vec2::uniform(0.08));
            self.save_model("seagrass", seagrass);
        }

        let mut fish = square.clone();
        if let Some(texture) = self.texture_handler.load_static_texture("fish.png") {
            fish.set_texture(texture);
        }
        fish.set_scale(Vec2::uniform(0.04));

        self.save_model("square", square);
        self.save_model("bubble", bubble);
        self.save_model("fish", fish);
    }

    pub fn load_fonts(&mut self) {
        if let Some(font) = self.texture_handler.load_font("Raleway-Regular.ttf") {
            self.save_font("default", font);
        }
    }

    fn save_model(&mut self, name: &str, model: Model) {
        self.models.insert(name.to_string(), model);
    }

    fn save_font(&mut self, name: &str, font: FontTexture) {
        self.fonts.insert(name.to_string(), font);
    }

    pub fn get_model(&self, name: &str) -> Model {
        if let Some(model) = self.models.get(name) {
            model.clone()
        } else {
            self.place_holder_model.clone()
        }
    }

    pub fn get_font(&self, name: &str) -> FontTexture {
        if let Some(font) = self.fonts.get(name) {
            font.clone()
        } else {
            self.place_holder_font.clone()
        }
    }

    pub fn get_texture_handler_mut(&mut self) -> &mut TextureHandler {
        &mut self.texture_handler
    }
}
