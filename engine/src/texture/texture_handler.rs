use gl::types::*;
use image::{imageops, GenericImageView};
use include_assets::NamedArchive;
use rusttype::{Font, Point, Scale};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::references;

use super::{
    font_texture::{FontTexture, OffsetTexture},
    texture::{AnimatedTexture, StaticTexture, Texture},
};

pub struct TextureHandler {
    id_map: HashMap<PathBuf, Texture>,
}

impl TextureHandler {
    pub fn new() -> Self {
        TextureHandler {
            id_map: HashMap::new(),
        }
    }

    pub fn load_static_texture(
        &mut self,
        archive: &NamedArchive,
        texture_name: &str,
    ) -> Option<Texture> {
        let path = Path::new(references::TEXTURES_PATH).join(texture_name.replace("/", "\\"));
        let binding = path.to_string_lossy().replace("/", "\\");
        let path_str = binding.as_str();

        let asset = archive
            .get(path_str)
            .expect(format!("Failed to load texture {:?}", path_str).as_str());
        if let Some(texture) = self.id_map.get(&path) {
            return Some(texture.clone());
        }
        let mut img = match image::load_from_memory(asset) {
            Ok(img) => img,
            Err(err) => {
                println!(
                    "Could not load image {}: {}",
                    path.to_string_lossy().to_string().as_str(),
                    err
                );
                return None;
            }
        };
        //let mut img = img.unwrap();
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
        return Some(texture);
    }

    pub fn load_font(&mut self, archive: &NamedArchive, font_name: &str) -> Option<FontTexture> {
        let path = Path::new(references::FONTS_PATH).join(font_name.replace("/", "\\"));
        let binding = path.to_string_lossy().replace("/", "\\");
        let path_str = binding.as_str();

        let asset = archive
            .get(path_str)
            .expect(format!("Failed to load font {:?}", path_str).as_str());

        let ft_opt = Font::try_from_bytes(asset);
        if ft_opt.is_none() {
            println!("Could not parse font file: {}", path_str);
            return None;
        }
        let font = ft_opt.unwrap();
        let resoluton = 128.0;
        let scale = Scale::uniform(resoluton);
        let v_metrics = font.v_metrics(scale);
        let mut font_texture = FontTexture::new();
        for c in 1 as char..=255 as char {
            let glyph = font.glyph(c).scaled(scale).positioned(Point::default());
            let bb_opt = glyph.pixel_bounding_box();

            if bb_opt.is_none() {
                font_texture.set(
                    c,
                    OffsetTexture {
                        texture: None,
                        v_offset: 0.0,
                        h_offset: glyph.unpositioned().h_metrics().advance_width,
                    },
                );
                continue;
            }

            let bb = bb_opt.unwrap();
            let width = bb.width() as usize;
            let height = bb.height() as usize;

            let buffer_size = width * height * 8;
            let mut buffer = vec![0u8; buffer_size];

            let scale = 0.9;
            let x_offset = (width as f32 * (1.0 - scale)) / 2.0;
            let y_offset = (height as f32 * (1.0 - scale)) / 2.0;

            glyph.draw(|x, y, v| {
                let scaled_x = (x as f32 * scale + x_offset) as u32;
                let scaled_y = (y as f32 * scale + y_offset) as u32;
                let index = (scaled_x as usize + (height - scaled_y as usize) * width) * 4;
                let intensity = (v * 255.0) as u8;
                buffer[index] = 255;
                buffer[index + 1] = 255;
                buffer[index + 2] = 255;
                buffer[index + 3] = intensity;
            });

            let mut id: u32 = 0;
            unsafe {
                gl::GenTextures(1, &mut id);
                gl::BindTexture(gl::TEXTURE_2D, id);
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                gl::TexParameteri(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_MIN_FILTER,
                    gl::LINEAR_MIPMAP_LINEAR as GLint,
                );
                gl::TexParameteri(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_MAG_FILTER,
                    gl::LINEAR_MIPMAP_LINEAR as GLint,
                );
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
                    buffer.as_ptr() as *const std::ffi::c_void,
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            };
            let texture: Texture = StaticTexture::new(id, width as u32, height as u32).into();
            let offset_y = v_metrics.ascent + bb.min.y as f32;

            let lsb = glyph.unpositioned().h_metrics().left_side_bearing;

            font_texture.set(
                c,
                OffsetTexture {
                    texture: texture.into(),
                    v_offset: offset_y,
                    h_offset: glyph.unpositioned().h_metrics().advance_width - (width as f32 + lsb),
                },
            );
        }

        Some(font_texture)
    }

    pub fn load_animated_texture(
        &mut self,
        archive: &NamedArchive,
        texture_names: &[&str],
        animation_time: u128,
    ) -> Option<Texture> {
        let mut static_textures: Vec<StaticTexture> = Vec::new();
        for texture_name in texture_names {
            if let Some(Texture::StaticTexture(static_texture)) =
                self.load_static_texture(archive, texture_name)
            {
                static_textures.push(static_texture);
            } else {
                return None;
            }
        }
        return Some(AnimatedTexture::new(static_textures, animation_time).into());
    }
}
