use winit::event_loop::EventLoopBuilder;

use lumina_engine::start;

fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    start(event_loop);
}
