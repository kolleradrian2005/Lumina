use std::{collections::HashMap};

use crate::engine::{gui::ui_model::UiModel, model::sprite, texture::texture::Texture};

#[derive(Clone)]
pub struct OffsetTexture {
    pub texture: Option<Texture>,
    pub v_offset: f32,
    pub h_offset: f32, // Offset of next glyph
}

#[derive(Clone)]
pub struct FontTexture {
    texture_ids: HashMap<char, OffsetTexture>,
}

impl FontTexture {
    pub fn new() -> FontTexture {
        Self {
            texture_ids: HashMap::new(),
        }
    }

    pub fn set(&mut self, c: char, texture: OffsetTexture) {
        self.texture_ids.insert(c, texture);
    }

    pub fn get(&self, c: char) -> OffsetTexture {
        if let Some(texture) = self.texture_ids.get(&c) {
            return texture.clone();
        }
        OffsetTexture {
            texture: None, //StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into(),
            v_offset: 0.0,
            h_offset: 0.0,
        }
    }

    pub fn get_model(&self, c: char) -> UiModel {
        let font_texture = self.get(c);
        let mut model = sprite::square(1.0);
        if let Some(texture) = &font_texture.texture {
            model.set_texture(texture.clone());
        }
        let (mut width, mut height) = (1, 1);
        if let Some(Texture::StaticTexture(texture)) = font_texture.texture {
            (width, height) = texture.get_dimensions();
        }
        UiModel {
            model,
            size: (width as f32, height as f32),
            margin: (font_texture.h_offset, font_texture.v_offset),
        }
    }
}
