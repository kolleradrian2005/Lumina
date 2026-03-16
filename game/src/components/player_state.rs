use lumina_engine::{logic::ecs::component::component::Component, math::vec3::Vec3};

#[derive(Component, Clone)]
pub enum PlayerState {
    Idle,
    Swimming { direction: Vec3 },
    FastSwimming { direction: Vec3 },
}

impl PlayerState {
    pub const fn direction(&self) -> Vec3 {
        match self {
            PlayerState::Idle => Vec3::zero(),
            PlayerState::Swimming { direction } => *direction,
            PlayerState::FastSwimming { direction } => *direction,
        }
    }
    pub const fn cam_zoom(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming { direction: _ } => 0.02,
            PlayerState::FastSwimming { direction: _ } => 0.05,
        }
    }
    pub const fn acceleration(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming { direction: _ } => 1.0,
            PlayerState::FastSwimming { direction: _ } => 1.25,
        }
    }
    pub const fn is_swimming(&self) -> bool {
        match self {
            PlayerState::Idle => false,
            PlayerState::Swimming { direction: _ } => true,
            PlayerState::FastSwimming { direction: _ } => true,
        }
    }
    pub const fn legs_animation_time(&self) -> u128 {
        match self {
            PlayerState::Idle => 0,
            PlayerState::Swimming { direction: _ } => 350,
            PlayerState::FastSwimming { direction: _ } => 300,
        }
    }
    pub const fn light_level(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.225,
            PlayerState::Swimming { direction: _ } => 0.175,
            PlayerState::FastSwimming { direction: _ } => 0.1,
        }
    }
}
