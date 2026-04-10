use std::collections::VecDeque;

use lumina_engine::{
    logic::{
        ecs::{component::transform::Transform, entity::entity::Entity, system::system::System},
        scene::world::World,
    },
    math::vec3::Vec3,
    shared::input::input_state::InputState,
};
use rand::{rngs::StdRng, Rng};
use winit::keyboard::Key;

use crate::{
    fish::{fish::Fish, fish_prefab::FishPrefab},
    player::player_state::PlayerState,
    sea_trash::sea_trash_prefab::SeaTrashPrefab,
};

pub struct BatchSpawnSystem {
    pub spawned_fish: VecDeque<Entity>,
    pub spawned_sea_trash: VecDeque<Entity>,
}

impl BatchSpawnSystem {
    pub fn new() -> Self {
        Self {
            spawned_fish: VecDeque::new(),
            spawned_sea_trash: VecDeque::new(),
        }
    }
}

impl System for BatchSpawnSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (transform, _)) in world.query_mut::<(&mut Transform, &mut PlayerState)>() {
            let rng = world.expect_resource_ptr::<StdRng>();
            if let Some(input_state) = world.get_resource::<InputState>() {
                if input_state.is_pressed(Key::Character("f".into())) {
                    self.spawn_fish(world, transform, rng);
                } else if input_state.is_pressed(Key::Character("t".into())) {
                    self.spawn_sea_trash(world, transform, rng);
                }
            }
        }
    }
}

const FISH_BATCH_SIZE: usize = 25;
const FISH_MAX_COUNT: usize = 500;

const SEA_TRASH_BATCH_SIZE: usize = 5;
const SEA_TRASH_MAX_COUNT: usize = 15;

impl BatchSpawnSystem {
    fn spawn_fish(&mut self, world: &mut World, transform: &mut Transform, rng: *mut StdRng) {
        for _ in 0..FISH_BATCH_SIZE {
            let random_offset = Vec3::new(
                (unsafe { (*rng).gen::<f32>() } - 0.5) * 2.0,
                (unsafe { (*rng).gen::<f32>() } - 0.5) * 2.0,
                (unsafe { (*rng).gen::<f32>() } - 0.5) * 1.5,
            );
            let is_flipped = unsafe { (*rng).gen::<bool>() };
            let speed_bias = (unsafe { (*rng).gen::<f32>() } - 0.5) * 0.03;
            let spawn_position = transform.position + random_offset;
            let fish_entity = FishPrefab::spawn(world);
            world
                .get_component_mut::<Transform>(fish_entity)
                .map(|fish_transform| {
                    fish_transform.position = spawn_position;
                    fish_transform.is_flipped = is_flipped;
                });
            world.get_component_mut::<Fish>(fish_entity).map(|fish| {
                fish.speed += speed_bias;
            });
            self.spawned_fish.push_back(fish_entity);
        }
        while self.spawned_fish.len() > FISH_MAX_COUNT {
            if let Some(old_entity) = self.spawned_fish.pop_front() {
                world.delete_entity(old_entity);
            }
        }
    }

    fn spawn_sea_trash(&mut self, world: &mut World, transform: &mut Transform, rng: *mut StdRng) {
        for _ in 0..SEA_TRASH_BATCH_SIZE {
            let random_offset = Vec3::new(
                (unsafe { (*rng).gen::<f32>() } - 0.5) * 2.0,
                (unsafe { (*rng).gen::<f32>() } - 0.5) * 2.0,
                0.0,
            );
            let is_flipped = unsafe { (*rng).gen::<bool>() };
            let spawn_position = transform.position + random_offset;
            let sea_trash_entity = SeaTrashPrefab::spawn(world);
            world
                .get_component_mut::<Transform>(sea_trash_entity)
                .map(|sea_trash_transform| {
                    sea_trash_transform.position = spawn_position;
                    sea_trash_transform.is_flipped = is_flipped;
                });
            self.spawned_sea_trash.push_back(sea_trash_entity);
        }
        while self.spawned_sea_trash.len() > SEA_TRASH_MAX_COUNT {
            if let Some(old_entity) = self.spawned_sea_trash.pop_front() {
                world.delete_entity(old_entity);
            }
        }
    }
}
