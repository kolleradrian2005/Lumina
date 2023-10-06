    use std::collections::HashMap;
    use image::{GenericImageView, imageops};
    use gl::types::*;

    use crate::{texture::Texture, references};

    pub struct TextureHandler {
        id_map: HashMap<String, Texture>
    }

    impl TextureHandler {
        pub fn new() -> Self {
            TextureHandler {
                id_map: HashMap::new()
            }
        }

        pub fn get_texture(&mut self, texture_name: &str) -> &Texture {
            return self.id_map.get(texture_name).unwrap();
        }

        pub fn load_texture(&mut self, texture_name: &str) -> bool {
            let path: String = (String::new() + references::TEXTURES_PATH + texture_name).to_string();
            if self.id_map.contains_key(&path) {
                return true;
            }
            let img = image::open(&path);
            if img.is_err() {
                println!("Could not load image: {}", path);
                return false;
            }
            let mut img = img.unwrap();
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
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as GLint,
                    width as GLsizei,
                    height as GLsizei,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    image_data.as_ptr() as * const std::ffi::c_void
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            };
            let texture = Texture::new(id);
            self.id_map.insert(texture_name.to_string(), texture);
            return true;
        }
    }