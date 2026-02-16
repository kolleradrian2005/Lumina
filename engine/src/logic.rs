use std::time::{Duration, Instant};

use rand::{rngs::StdRng, SeedableRng};

use crate::{
    input::input_event::InputEvent,
    render::render_packet::RenderPacket,
    scene::scene::Scene,
};

const TARGET_INTERVAL: Duration = Duration::from_micros(16666);

pub async fn run_logic_loop(
    input_rx: flume::Receiver<InputEvent>,
    render_tx: flume::Sender<RenderPacket>,
    mut scene: Scene,
) {
    let rng: StdRng = SeedableRng::from_entropy();
    let mut delta_time: Duration;
    let mut last: Instant = Instant::now();

    /*world.insert_resource(render_ctx.resource_handle.get_inner());
    world.insert_resource(loop_input_state_clone.clone());
    world.insert_resource(updatables_clone.clone());
    world.insert_resource(drop_mesh_requests_clone.clone());
    world.insert_resource(create_mesh_manager_clone.clone());*/
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
            scene.handle_input_event(event);
        }
        scene.update(delta_time.as_secs_f32());

        let packet = scene.get_world().render_packet.clone();
        let _ = render_tx.send(packet);
        scene.get_world_mut().clear_render_packet();
    }
}
