use std::collections::VecDeque;

use noise::{NoiseFn, Perlin};

use crate::{model::Model, vec2::Vec2};

extern crate noise;

pub struct Terrain {
    noise: Perlin,
    pub tile_size: f32,
    loaded_tiles: VecDeque<Model>,
    loaded_tile_index: i32,
    default_tile_count: i32
}

impl Terrain {
    pub fn new(seed: u32) -> Self {
        let noise = Perlin::new(seed);
        let tile_size = 0.1;
        let loaded_tiles = VecDeque::new();
        let loaded_tile_index: i32 = 0;
        let mut default_tile_count = (2.0 / tile_size as f32) as i32 + 2;
        default_tile_count -= default_tile_count % 2;
        let mut terrain = Terrain { noise, tile_size, loaded_tiles, loaded_tile_index, default_tile_count };
        terrain.loaded_tiles.push_back(terrain.generate_tile(loaded_tile_index));
        for i in 1..default_tile_count / 2 + 1 as i32 {
            terrain.loaded_tiles.push_back(terrain.generate_tile(loaded_tile_index + i));
            terrain.loaded_tiles.push_front(terrain.generate_tile(loaded_tile_index - i));
        }
        terrain
    }
    fn generate_tile(&self, x: i32) -> Model {
        let previous_y: f32 = self.get_height(x - 1);
        let current_y: f32 = self.get_height(x);
        let next_y: f32 = self.get_height(x + 1);
        let left_y_offset = (previous_y - current_y) / 2.0;
        let right_y_offset = (current_y - next_y) / 2.0;
        let vertices: &[f32] = &[
            // Bottom left
            -self.tile_size / 2.0, -1.0 + left_y_offset, 1.0,
            // Bottom right
            self.tile_size / 2.0, -1.0 - right_y_offset, 1.0,
            // Top right
            self.tile_size / 2.0, self.tile_size / 2.0 - right_y_offset, 1.0,
            // Top left
            -self.tile_size / 2.0, self.tile_size / 2.0 + left_y_offset, 1.0,
        ];

        let indices: &[u32] = &[
            0, 1, 2,
            2, 3, 0
        ];

        let uvs: &[f32] = &[
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];
        let mut tile: Model = Model::new(vertices, indices, uvs);
        tile.set_position(Vec2::new(x as f32 * self.tile_size, current_y));
        tile
    }
    pub fn get_height(&self, x: i32) -> f32 {
        let amplitude = 0.1;
        let frequency = 0.2;
        let noise_value = self.noise.get([frequency * x as f64, 0.0]) * amplitude;
        // TODO: add more noise to create hills
        noise_value as f32
    }

    pub fn get_loaded_tiles(&self) -> &VecDeque<Model> {
        &self.loaded_tiles
    }

    pub fn update_tile_index(&mut self, tile_index: i32) {
        let difference = self.loaded_tile_index - tile_index; 
        if difference != 0 {
            self.loaded_tile_index = tile_index;
            if difference > 0 {
                self.sweep_left();
            } else {
                self.sweep_right();
            }
        }
    }

    fn sweep_left(&mut self) {
        let new_tile = self.generate_tile((self.loaded_tile_index - self.default_tile_count / 2) as i32);
        self.loaded_tiles.push_front(new_tile);
        self.loaded_tiles.pop_back();
    }
    fn sweep_right(&mut self) {
        let new_tile = self.generate_tile((self.loaded_tile_index + self.default_tile_count / 2) as i32);
        self.loaded_tiles.push_back(new_tile);
        self.loaded_tiles.pop_front();
    }
}
