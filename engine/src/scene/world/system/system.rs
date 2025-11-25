use crate::scene::world::world::World;

pub trait System: Send + Sync {
    fn run(&self, world: &mut World, delta_time: f32);
}
