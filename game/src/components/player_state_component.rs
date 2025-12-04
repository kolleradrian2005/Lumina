use lumina_engine::{math::vec3::Vec3, scene::world::component::component::Component};

#[derive(Component, Clone)]
pub enum PlayerStateComponent {
    Idle,
    Swimming { direction: Vec3 },
    FastSwimming { direction: Vec3 },
}

impl PlayerStateComponent {
    pub const fn direction(&self) -> Vec3 {
        match self {
            PlayerStateComponent::Idle => Vec3::zero(),
            PlayerStateComponent::Swimming { direction } => *direction,
            PlayerStateComponent::FastSwimming { direction } => *direction,
        }
    }
    pub const fn cam_zoom(&self) -> f32 {
        match self {
            PlayerStateComponent::Idle => 0.0,
            PlayerStateComponent::Swimming { direction: _ } => 0.02,
            PlayerStateComponent::FastSwimming { direction: _ } => 0.05,
        }
    }
    pub const fn acceleration(&self) -> f32 {
        match self {
            PlayerStateComponent::Idle => 0.0,
            PlayerStateComponent::Swimming { direction: _ } => 1.0,
            PlayerStateComponent::FastSwimming { direction: _ } => 1.25,
        }
    }
    pub const fn is_swimming(&self) -> bool {
        match self {
            PlayerStateComponent::Idle => false,
            PlayerStateComponent::Swimming { direction: _ } => true,
            PlayerStateComponent::FastSwimming { direction: _ } => true,
        }
    }
    pub const fn legs_animation_time(&self) -> u128 {
        match self {
            PlayerStateComponent::Idle => 0,
            PlayerStateComponent::Swimming { direction: _ } => 350,
            PlayerStateComponent::FastSwimming { direction: _ } => 300,
        }
    }
    pub const fn light_level(&self) -> f32 {
        match self {
            PlayerStateComponent::Idle => 0.225,
            PlayerStateComponent::Swimming { direction: _ } => 0.175,
            PlayerStateComponent::FastSwimming { direction: _ } => 0.1,
        }
    }
}
