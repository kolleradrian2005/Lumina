use std::time::{Duration, Instant};

use flume::{Receiver, Sender};
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    logic::scene::scene::Scene,
    shared::{
        extracted_frame::ExtractedFrame,
        input::{input_event::InputEvent, input_handler::InputHandler},
    },
};

const TARGET_INTERVAL: Duration = Duration::from_micros(16666);

pub async fn run_logic_loop(
    input_rx: Receiver<InputEvent>,
    render_tx: Sender<ExtractedFrame>,
    mut scene: Scene,
) {
    let rng: StdRng = SeedableRng::from_entropy();
    let mut delta_time: Duration;
    let mut last: Instant = Instant::now();
    let world = scene.get_world_mut();
    world.insert_resource(rng);

    loop {
        delta_time = last.elapsed();
        if delta_time < TARGET_INTERVAL {
            spin_sleep::sleep(TARGET_INTERVAL - delta_time);
        }
        delta_time = Duration::max(delta_time, TARGET_INTERVAL);
        last = Instant::now();

        while let Ok(event) = input_rx.try_recv() {
            let world = scene.get_world_mut();
            InputHandler::handle_input_event(world, event);
        }
        scene.update(delta_time.as_secs_f32());

        let frame = scene.extract();
        let _ = render_tx.send(frame);
    }
}
