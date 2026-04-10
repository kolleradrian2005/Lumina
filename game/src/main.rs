pub mod camera;
pub mod fish;
pub mod game;
pub mod player;
pub mod postprocess;
pub mod scene;
pub mod sea_trash;

use winit::event_loop::EventLoopBuilder;

fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    game::initialize(event_loop);
}
