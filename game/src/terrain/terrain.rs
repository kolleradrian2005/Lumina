use std::{collections::VecDeque, f32::consts::PI};

use lumina_engine::{
    math::vec3::Vec3,
    scene::world::world::World,
    texture::{
        resource_manager::ResourceManager,
        texture::{StaticColor, Texture},
    },
};
use noise::{NoiseFn, Perlin};

use crate::terrain::tile::Tile;

pub struct Terrain {
    pub noise: Perlin,
    tile_size: f32,
    pub tiles: VecDeque<Tile>,
    pub loaded_tile_index: i32,
    default_tile_count: i32,
    tile_texture: Texture,
}

impl Terrain {
    pub fn generate(world: &mut World, seed: u32, resource_manager: &mut ResourceManager) -> Self {
        let tile_size = 0.5;
        let extra_tiles = 4;
        let mut default_tile_count = (2.0 / tile_size as f32) as i32 + extra_tiles;
        default_tile_count -= default_tile_count % 2;
        let loaded_tile_index = 0;
        let mut terrain = Terrain {
            noise: Perlin::new(seed),
            tiles: VecDeque::new(),
            tile_size,
            loaded_tile_index,
            default_tile_count, // Even
            tile_texture: StaticColor::new((0.8235, 0.7059, 0.5490).into()).into(),
        };
        terrain
            .tiles
            .push_back(terrain.generate_tile(world, loaded_tile_index, resource_manager));
        for i in 1..default_tile_count / 2 + 1 as i32 {
            terrain.tiles.push_back(terrain.generate_tile(
                world,
                loaded_tile_index + i,
                resource_manager,
            ));
            terrain.tiles.push_front(terrain.generate_tile(
                world,
                loaded_tile_index - i,
                resource_manager,
            ));
        }
        terrain
    }

    pub fn get_tile_size(&self) -> f32 {
        self.tile_size
    }

    pub fn get_default_tile_count(&self) -> i32 {
        self.default_tile_count
    }

    pub fn get_tile_texture(&self) -> &Texture {
        &self.tile_texture
    }

    pub fn is_loaded(&self, pos: Vec3) -> bool {
        return ((pos.x / self.tile_size).round() as i32 - self.loaded_tile_index).abs()
            <= self.default_tile_count / 2;
    }

    pub fn get_tiles(&self) -> &VecDeque<Tile> {
        &self.tiles
    }

    pub fn get_tiles_mut(&mut self) -> &mut VecDeque<Tile> {
        &mut self.tiles
    }

    pub fn get_height(&self, x_f: f32) -> f32 {
        let x = (x_f / self.tile_size + 0.5).floor() as i32;
        let mut x_fract = (x_f / self.tile_size + 0.5).fract();
        if x_fract < 0.0 {
            x_fract = 1.0 - x_fract;
        }

        let previous_y: f32 = Terrain::get_height_noise(x - 1, &self.noise);
        let current_y: f32 = Terrain::get_height_noise(x, &self.noise);
        let next_y: f32 = Terrain::get_height_noise(x + 1, &self.noise);

        let left_y_offset = (previous_y - current_y) / 2.0;
        let right_y_offset = (current_y - next_y) / 2.0;
        let a = self.tile_size / 2.0 + left_y_offset;
        let b = self.tile_size / 2.0 - right_y_offset;
        let uphill = a < b;

        let top = f32::max(previous_y, next_y);
        let bot = f32::min(previous_y, next_y);

        let height = top - bot;

        current_y + bot + height * Self::interpolate(f32::from(!uphill), f32::from(uphill), x_fract)
    }

    pub fn interpolate(a: f32, b: f32, blend: f32) -> f32 {
        let tetha = blend * PI;
        let f = (1.0 - f32::cos(tetha)) * 0.5;
        a * (1.0 - f) + b * f
    }

    pub fn get_fish_noise(x: i32, noise: &Perlin) -> f32 {
        let amplitude = 1.5;
        let frequency = 0.15;
        let offset = 8.0;
        let mut noise_value = noise.get([frequency * x as f64, offset]) * amplitude;
        noise_value += noise.get([frequency / 2.0 * x as f64, offset]) * amplitude * 1.5;
        noise_value as f32
    }

    pub fn get_seaweed_noise(x: i32, noise: &Perlin) -> f32 {
        let amplitude = 1.5;
        let frequency = 0.15;
        let offset = 4.0;
        let mut noise_value = noise.get([frequency * x as f64, offset]) * amplitude;
        noise_value += noise.get([frequency / 2.0 * x as f64, offset]) * amplitude * 1.5;
        noise_value as f32
    }

    pub fn get_height_noise(x: i32, noise: &Perlin) -> f32 {
        let amplitude = 0.15;
        let frequency = 0.2;
        let mut noise_value = noise.get([frequency * x as f64, 0.0]) * amplitude;
        noise_value += noise.get([frequency / 4.0 * x as f64, 0.0]) * amplitude * 2.0;
        noise_value as f32
    }

    fn generate_tile(
        &self,
        world: &mut World,
        x: i32,
        resource_manager: &mut ResourceManager,
    ) -> Tile {
        Tile::generate(
            world,
            self.tile_size,
            x,
            &self.noise,
            &self.tile_texture,
            resource_manager,
        )
    }
}
