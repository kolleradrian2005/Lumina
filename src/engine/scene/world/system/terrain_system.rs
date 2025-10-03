use std::sync::{Arc, Mutex};

use crate::engine::{
    render::renderable::MeshLoadState,
    scene::{
        terrain::Terrain,
        tile::Tile,
        world::{
            component::{
                model_component::ModelComponent, player_state_component::PlayerStateComponent,
                transform_component::TransformComponent,
            },
            world::World,
        },
    },
    texture::resource_provider::ResourceProvider,
};

use super::system::System;

pub struct TerrainSystem;

impl System for TerrainSystem {
    fn run(&self, world: &mut World, _: f32) {
        let terrain = world.expect_resource::<Arc<Mutex<Terrain>>>().clone();
        let resource_provider = world
            .expect_resource::<Arc<Mutex<dyn ResourceProvider>>>()
            .clone();
        for (_, (_, transform)) in
            world.query_mut::<(&mut PlayerStateComponent, &mut TransformComponent)>()
        {
            if let (Ok(terrain), Ok(resource_provider)) =
                (&mut terrain.lock(), &mut resource_provider.lock())
            {
                let tile_index = (transform.position.x / terrain.get_tile_size()).round() as i32;
                Self::update_tile_index(world, terrain, &mut **resource_provider, tile_index);
            }
        }
    }
}

impl TerrainSystem {
    fn update_tile_index(
        world: &mut World,
        terrain: &mut Terrain,
        resource_provider: &mut dyn ResourceProvider,
        tile_index: i32,
    ) {
        let difference = terrain.loaded_tile_index - tile_index;
        if difference != 0 {
            terrain.loaded_tile_index = tile_index;
            match difference > 0 {
                true => Self::sweep_left(world, terrain, resource_provider),
                false => Self::sweep_right(world, terrain, resource_provider),
            }
        }
    }

    fn sweep_left(
        world: &mut World,
        terrain: &mut Terrain,
        resource_provider: &mut dyn ResourceProvider,
    ) {
        let new_tile = Tile::generate(
            world,
            terrain.get_tile_size(),
            (terrain.loaded_tile_index - terrain.get_default_tile_count() / 2) as i32,
            &terrain.noise,
            terrain.get_tile_texture(),
            resource_provider,
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
        resource_provider: &mut dyn ResourceProvider,
    ) {
        let new_tile = Tile::generate(
            world,
            terrain.get_tile_size(),
            (terrain.loaded_tile_index + terrain.get_default_tile_count() / 2) as i32,
            &terrain.noise,
            terrain.get_tile_texture(),
            resource_provider,
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
