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
pub mod components {
    pub mod conditional_parent_component;
    pub mod follow_component;
    pub mod multi_conditional_parent_component;
    pub mod player_part_component;
    pub mod player_state_component;
}
pub mod terrain {
    pub mod terrain;
    pub mod tile;
    pub mod water;
}
pub mod game;
pub mod player_state;

use winit::event_loop::EventLoopBuilder;

fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    game::initialize(event_loop);
}
