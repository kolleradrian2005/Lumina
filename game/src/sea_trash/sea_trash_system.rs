use lumina_engine::logic::{
    ecs::{component::collider::Collider, system::system::System},
    scene::world::World,
};

use crate::sea_trash::sea_trash::SeaTrash;

pub struct SeaTrashSystem;

impl System for SeaTrashSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (_sea_trash, _collider)) in world.query_mut::<(&mut SeaTrash, &mut Collider)>() {}
    }
}
