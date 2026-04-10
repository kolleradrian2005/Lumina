use gl::types::*;
use image::{imageops, GenericImageView};
use include_assets::NamedArchive;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    engine_config,
    render::resource::texture::texture::{AnimatedTexture, StaticTexture, Texture},
    shared::engine_error::EngineError,
};

pub struct TextureLoader {
    id_map: HashMap<PathBuf, Texture>,
}

impl TextureLoader {
    pub fn new() -> Self {
        TextureLoader {
            id_map: HashMap::new(),
        }
    }

    pub fn load_static_texture(
        &mut self,
        archive: &NamedArchive,
        texture_name: &str,
    ) -> Result<Texture, EngineError> {
        let path = Path::new(engine_config::TEXTURES_PATH).join(texture_name.replace("/", "\\"));
        let binding = path.to_string_lossy().replace("/", "\\");
        let path_str = binding.as_str();

        if let Some(texture) = self.id_map.get(&path) {
            return Ok(texture.clone());
        }

        let asset = archive.get(path_str);
        if asset.is_none() {
            return Err(EngineError::FileNotFound(path_str.to_string()));
        }
        let mut img = match image::load_from_memory(asset.unwrap()) {
            Ok(img) => img,
            Err(err) => {
                return Err(EngineError::Generic(format!(
                    "Could not load image '{}': {}",
                    path.to_string_lossy().to_string().as_str(),
                    err
                )));
            }
        };
        imageops::flip_vertical_in_place(&mut img);
        let (width, height) = img.dimensions();
        let binding = img.to_rgba8();
        let image_data = binding.as_raw();
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::MIRRORED_REPEAT as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::MIRRORED_REPEAT as GLint,
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        };
        let texture: Texture = StaticTexture::new(id, width, height).into();
        self.id_map.insert(path, texture.clone());
        return Ok(texture);
    }

    pub fn load_animated_texture(
        &mut self,
        archive: &NamedArchive,
        texture_names: &[String],
        animation_time: u128,
    ) -> Result<Texture, EngineError> {
        let mut static_textures: Vec<StaticTexture> = Vec::new();
        for texture_name in texture_names {
            if let Ok(Texture::StaticTexture(static_texture)) =
                self.load_static_texture(archive, texture_name)
            {
                static_textures.push(static_texture);
            } else {
                return Err(EngineError::Generic(format!(
                    "Failed to load static texture: {}",
                    texture_name
                )));
            }
        }
        return Ok(AnimatedTexture::new(static_textures, animation_time).into());
    }
}
