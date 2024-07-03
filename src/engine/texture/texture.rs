use std::time::Instant;

use crate::engine::math::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum Texture {
    StaticColor(StaticColor),
    StaticTexture(StaticTexture),
    AnimatedTexture(AnimatedTexture),
    GradientTexture(GradientTexture),
}

impl From<StaticColor> for Texture {
    fn from(static_color: StaticColor) -> Texture {
        Texture::StaticColor(static_color)
    }
}

impl From<StaticTexture> for Texture {
    fn from(static_texture: StaticTexture) -> Texture {
        Texture::StaticTexture(static_texture)
    }
}

impl From<AnimatedTexture> for Texture {
    fn from(animated_texture: AnimatedTexture) -> Texture {
        Texture::AnimatedTexture(animated_texture)
    }
}

impl From<GradientTexture> for Texture {
    fn from(gradient_texture: GradientTexture) -> Texture {
        Texture::GradientTexture(gradient_texture)
    }
}

impl Texture {
    pub const fn has_texture(&self) -> bool {
        match self {
            Texture::StaticColor(_) => false,
            Texture::StaticTexture(_) => true,
            Texture::AnimatedTexture(_) => true,
            Texture::GradientTexture(_) => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StaticColor {
    pub color: Vec3,
}

impl StaticColor {
    pub fn new(color: Vec3) -> Self {
        StaticColor { color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StaticTexture {
    id: u32,
    width: u32,
    height: u32,
}

impl StaticTexture {
    pub fn new(id: u32, width: u32, height: u32) -> Self {
        StaticTexture { id, width, height }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_normalized_dimensions(&self) -> (f32, f32) {
        let ratio = self.width as f32 / self.height as f32;
        let mut a = ratio;
        let mut b = 1.0;
        if 1.0 < ratio {
            a = 1.0;
            b /= ratio;
        }
        (a, b)
    }
}

#[derive(Clone, Debug)]
pub struct AnimatedTexture {
    pub textures: Vec<StaticTexture>,
    pub animation_time: u128,
    pub animation_start_time: Instant,
}

impl AnimatedTexture {
    pub fn new(textures: Vec<StaticTexture>, animation_time: u128) -> Self {
        AnimatedTexture {
            textures,
            animation_time,
            animation_start_time: Instant::now(),
        }
    }
    pub fn current_texture(&self) -> StaticTexture {
        let texture_count = self.textures.len();
        if self.animation_time == 0 {
            return *self.textures.get(0).unwrap();
        }
        let texture_index = ((Instant::now()
            .duration_since(self.animation_start_time)
            .as_millis()
            % self.animation_time) as f32
            / (self.animation_time as f32 / texture_count as f32))
            as usize;
        *self.textures.get(texture_index).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GradientTexture {
    pub color1: Vec3,
    pub color2: Vec3,
}

impl GradientTexture {
    pub fn new(color1: Vec3, color2: Vec3) -> Self {
        GradientTexture { color1, color2 }
    }
}
