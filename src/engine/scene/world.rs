use crate::engine::texture::resource_manager::ResourceManager;

use super::{terrain::Terrain, water::Water};

extern crate noise;

pub struct World {
    pub terrain: Terrain,
    water: Water,
}

impl World {
    pub fn load(seed: u32, resoure_manager: &mut ResourceManager) -> Self {
        World {
            terrain: Terrain::generate(seed, resoure_manager),
            water: Water::create((seed ^ 0x5EAF00D).wrapping_mul(69696969)),
        }
    }

    pub fn get_terrain(&self) -> &Terrain {
        &self.terrain
    }

    pub fn get_terrain_mut(&mut self) -> &mut Terrain {
        &mut self.terrain
    }

    pub fn get_water(&self) -> &Water {
        &self.water
    }
}
