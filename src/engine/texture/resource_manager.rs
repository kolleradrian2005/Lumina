use std::collections::HashMap;

use include_assets::NamedArchive;

use crate::engine::{
    math::{vec2::Vec2, vec3::Vec3},
    model::{model::Model, model_group::ModelGroup, sprite},
    transformable::Transformable,
};

use super::{
    font_texture::FontTexture,
    resource_provider::ResourceProvider,
    texture::{StaticColor, Texture},
    texture_handler::TextureHandler,
};

pub struct ResourceManager {
    place_holder_model: Model,
    place_holder_model_group: ModelGroup,
    place_holder_font: FontTexture,
    models: HashMap<String, Model>,
    model_groups: HashMap<String, ModelGroup>,
    fonts: HashMap<String, FontTexture>,
    texture_handler: TextureHandler,
    archive: NamedArchive,
}

impl ResourceManager {
    pub fn new(archive: NamedArchive) -> Self {
        ResourceManager {
            place_holder_model: sprite::square(1.0),
            place_holder_model_group: ModelGroup::new(None),
            place_holder_font: FontTexture::new(),
            models: HashMap::new(),
            model_groups: HashMap::new(),
            fonts: HashMap::new(),
            texture_handler: TextureHandler::new(),
            archive,
        }
    }
}

impl ResourceProvider for ResourceManager {
    fn get_archive(&self) -> &NamedArchive {
        &self.archive
    }

    fn preload_models(&mut self) {
        let mut square = sprite::square(1.0);
        square.set_texture(StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into());
        let mut bubble = square.clone();
        bubble.set_scale(Vec2::uniform(0.01));
        if let Some(texture) = self
            .texture_handler
            .load_static_texture(&self.archive, "bubble.png")
        {
            bubble.set_texture(texture);
        }
        if let Some(Texture::StaticTexture(texture)) = self
            .texture_handler
            .load_static_texture(&self.archive, "seagrass0.png")
        {
            let mut seagrass = sprite::from_texture(texture);
            seagrass.set_scale(Vec2::uniform(0.08));
            self.save_model("seagrass", seagrass);
        }

        let mut fish = square.clone();
        if let Some(texture) = self
            .texture_handler
            .load_static_texture(&self.archive, "fish.png")
        {
            fish.set_texture(texture);
        }
        fish.set_scale(Vec2::uniform(0.04));

        let mut seagrass = self.load_seagrass(&[]);
        seagrass.set_scale(Vec2::uniform(0.08));

        self.save_model("square", square);
        self.save_model("bubble", bubble);
        self.save_model("fish", fish);
        self.save_model_group("seagrass", seagrass);
    }

    fn load_fonts(&mut self) {
        if let Some(font) = self
            .texture_handler
            .load_font(&self.archive, "Raleway-Regular.ttf")
        {
            self.save_font("default", font);
        }
    }

    fn save_model(&mut self, name: &str, model: Model) {
        self.models.insert(name.to_string(), model);
    }

    fn save_model_group(&mut self, name: &str, model_group: ModelGroup) {
        self.model_groups.insert(name.to_string(), model_group);
    }

    fn save_font(&mut self, name: &str, font: FontTexture) {
        self.fonts.insert(name.to_string(), font);
    }

    fn get_model(&self, name: &str) -> Model {
        if let Some(model) = self.models.get(name) {
            model.clone()
        } else {
            self.place_holder_model.clone()
        }
    }

    fn get_model_group(&self, name: &str) -> ModelGroup {
        if let Some(model_group) = self.model_groups.get(name) {
            model_group.clone()
        } else {
            self.place_holder_model_group.clone()
        }
    }

    fn get_font(&self, name: &str) -> FontTexture {
        if let Some(font) = self.fonts.get(name) {
            font.clone()
        } else {
            self.place_holder_font.clone()
        }
    }

    fn get_texture_handler_mut(&mut self) -> &mut TextureHandler {
        &mut self.texture_handler
    }

    fn load_static_texture(&mut self, texture_name: &str) -> Option<Texture> {
        self.texture_handler
            .load_static_texture(&self.archive, texture_name)
    }

    fn load_animated_texture(
        &mut self,
        texture_names: &[&str],
        animation_time: u128,
    ) -> Option<Texture> {
        self.texture_handler
            .load_animated_texture(&self.archive, texture_names, animation_time)
    }

    fn load_seagrass(&mut self, texture_names: &[&str]) -> ModelGroup {
        let mut model_group = ModelGroup::new(None);
        const MODEL_THICKNESS: f32 = 0.4;
        let z_step: f32 = -MODEL_THICKNESS / texture_names.len() as f32;
        let mut z = MODEL_THICKNESS / 2.0;
        for texture_name in texture_names {
            if let Some(Texture::StaticTexture(texture)) = self
                .texture_handler
                .load_static_texture(&self.archive, texture_name)
            {
                let mut model = sprite::from_texture(texture);
                model.set_position(Vec3::new(0.0, 0.0, z));
                z += z_step;
                model_group.add_model(model);
            }
        }
        model_group
    }
}
