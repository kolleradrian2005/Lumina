pub mod systems {
    pub mod animation_system;
    pub mod camera_system;
    pub mod current_system;
    pub mod follow_system;
    pub mod input_system;
    pub mod player_movement_system;
    pub mod terrain_collision_system;
    pub mod terrain_system;
    pub mod update_focal_radius_system;
    pub mod update_god_rays_system;
}
pub mod extractors {
    pub mod postprocess_buffer_extractor;
}
pub mod components {
    pub mod conditional_parent;
    pub mod current;
    pub mod follow;
    pub mod multi_conditional_parent;
    pub mod player_part;
    pub mod player_state;
}
pub mod terrain {
    pub mod terrain;
    pub mod tile;
    pub mod water;
}
pub mod foreground;
pub mod game;
pub mod object_type;
pub mod particle;
pub mod player_state_definition;
pub mod postprocess_uniform_buffer;

use winit::event_loop::EventLoopBuilder;

fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    game::initialize(event_loop);
}
