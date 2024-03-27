use crate::engine::texture::texture::{StaticTexture, Texture};

use super::model::Model;

pub static Z_DEFAULT: f32 = 0.0;

pub static INDICES: [u32; 6] = [
    0, 1, 2,
    2, 3, 0
];

pub static UVS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    0.0, 1.0
];

fn vertices(half_width: f32, half_height: f32) -> [f32; 12] {
    [
        -half_width, -half_height, Z_DEFAULT,
        half_width, -half_height, Z_DEFAULT,
        half_width, half_height, Z_DEFAULT,
        -half_width, half_height, Z_DEFAULT
    ]
}

pub fn rectangle(width: f32, height: f32) -> Model {
    Model::new(&self::vertices(width / 2.0, height / 2.0), &self::INDICES, &self::UVS)
}

pub fn square(size: f32) -> Model {
    self::rectangle(size, size)
}

pub fn from_texture(texture: StaticTexture) -> Model {
    let (width, height) = texture.get_normalized_dimensions();
    let mut model = self::rectangle(width, height);
    model.set_texture(Texture::StaticTexture(texture));
    model
}
