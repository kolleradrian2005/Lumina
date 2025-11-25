use crate::{
    math::vec3::Vec3,
    texture::texture::{StaticColor, Texture},
};

use super::component::Component;

#[derive(Clone, Debug)]
pub struct TextureComponent {
    pub texture: Texture,
}

impl Default for TextureComponent {
    fn default() -> Self {
        Self {
            texture: StaticColor::new(Vec3::new(0.5, 0.5, 0.5)).into(),
        }
    }
}

impl From<Texture> for TextureComponent {
    fn from(texture: Texture) -> Self {
        Self { texture }
    }
}

impl Component for TextureComponent {}
