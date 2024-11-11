use std::sync::Arc;

use crate::engine::{
    command_queue::CommandQueue,
    texture::texture::{StaticTexture, Texture},
};

use super::model::Model;

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

pub fn rectangle(command_queue: Arc<CommandQueue>, width: f32, height: f32) -> Model {
    Model::new(
        command_queue,
        &self::vertices(width / 2.0, height / 2.0),
        &self::INDICES,
        &self::UVS,
    )
}

pub fn square(command_queue: Arc<CommandQueue>, size: f32) -> Model {
    self::rectangle(command_queue, size, size)
}

pub fn from_texture(command_queue: Arc<CommandQueue>, texture: StaticTexture) -> Model {
    let (width, height) = texture.get_normalized_dimensions();
    let mut model = self::rectangle(command_queue, width, height);
    model.set_texture(Texture::StaticTexture(texture));
    model
}
