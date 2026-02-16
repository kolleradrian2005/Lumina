use std::sync::{Arc, Mutex};

use lumina_engine::{
    scene::{
        terrain::Terrain,
        tile::Tile,
        world::{
            component::transform_component::TransformComponent, system::system::System,
            world::World,
        },
    },
    texture::resource_manager::ResourceManager,
};

use crate::components::player_state_component::PlayerStateComponent;

pub struct TerrainSystem;

impl System for TerrainSystem {
    fn run(&self, world: &mut World, _: f32) {
        let terrain = world.expect_resource::<Arc<Mutex<Terrain>>>().clone();
        let resource_manager = world.expect_resource_ptr::<ResourceManager>();
        for (_, (_, transform)) in
            world.query_mut::<(&mut PlayerStateComponent, &mut TransformComponent)>()
        {
            if let Ok(terrain) = &mut terrain.lock() {
                let tile_index = (transform.position.x / terrain.get_tile_size()).round() as i32;
                Self::update_tile_index(
                    world,
                    terrain,
                    unsafe { &mut *resource_manager },
                    tile_index,
                );
            }
        }
    }
}

impl TerrainSystem {
    fn update_tile_index(
        world: &mut World,
        terrain: &mut Terrain,
        resource_manager: &mut ResourceManager,
        tile_index: i32,
    ) {
        let difference = terrain.loaded_tile_index - tile_index;
        if difference != 0 {
            terrain.loaded_tile_index = tile_index;
            match difference > 0 {
                true => Self::sweep_left(world, terrain, resource_manager),
                false => Self::sweep_right(world, terrain, resource_manager),
            }
        }
    }

    fn sweep_left(
        world: &mut World,
        terrain: &mut Terrain,
        resource_manager: &mut ResourceManager,
    ) {
        let new_tile = Tile::generate(
            world,
            terrain.get_tile_size(),
            (terrain.loaded_tile_index - terrain.get_default_tile_count() / 2) as i32,
            &terrain.noise,
            terrain.get_tile_texture(),
            resource_manager,
        );
        terrain.tiles.push_front(new_tile);
        terrain.tiles.pop_back().map(|tile: Tile| {
            world.delete_entity(*tile.get_entity());
            tile.get_objects()
                .iter()
                .for_each(|e| world.delete_entity(*e))
        });
    }

    fn sweep_right(
        world: &mut World,
        terrain: &mut Terrain,
        resource_manager: &mut ResourceManager,
    ) {
        let new_tile = Tile::generate(
            world,
            terrain.get_tile_size(),
            (terrain.loaded_tile_index + terrain.get_default_tile_count() / 2) as i32,
            &terrain.noise,
            terrain.get_tile_texture(),
            resource_manager,
        );
        terrain.tiles.push_back(new_tile);
        terrain.tiles.pop_front().map(|tile| {
            world.delete_entity(*tile.get_entity());
            tile.get_objects()
                .iter()
                .for_each(|e| world.delete_entity(*e))
        });
    }
}
