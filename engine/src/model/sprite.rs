use crate::texture::texture::StaticTexture;


pub static Z_DEFAULT: f32 = 0.0;

pub static INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

pub static UVS: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];

fn vertices(half_width: f32, half_height: f32) -> [f32; 12] {
    [
        -half_width,
        -half_height,
        Z_DEFAULT,
        half_width,
        -half_height,
        Z_DEFAULT,
        half_width,
        half_height,
        Z_DEFAULT,
        -half_width,
        half_height,
        Z_DEFAULT,
    ]
}

pub fn rectangle(width: f32, height: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    (
        self::vertices(width / 2.0, height / 2.0).to_vec(),
        self::INDICES.to_vec(),
        self::UVS.to_vec(),
    )
}

pub fn square(size: f32) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    self::rectangle(size, size)
}

pub fn from_texture(texture: &StaticTexture) -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let (width, height) = texture.get_normalized_dimensions();
    self::rectangle(width, height)
}
