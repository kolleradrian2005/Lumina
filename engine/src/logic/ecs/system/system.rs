use crate::logic::scene::world::World;

pub trait System: Send + Sync {
    fn run(&mut self, world: &mut World, delta_time: f32);
}
