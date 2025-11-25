#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Swimming,
    FastSwimming,
}

impl PlayerState {
    pub const fn zoom(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming => 0.02,
            PlayerState::FastSwimming => 0.05,
        }
    }
    pub const fn acceleration(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.0,
            PlayerState::Swimming => 1.0,
            PlayerState::FastSwimming => 1.25,
        }
    }
    pub const fn is_swimming(&self) -> bool {
        match self {
            PlayerState::Idle => false,
            PlayerState::Swimming => true,
            PlayerState::FastSwimming => true,
        }
    }
    pub const fn legs_animation_time(&self) -> u128 {
        match self {
            PlayerState::Idle => 0,
            PlayerState::Swimming => 350,
            PlayerState::FastSwimming => 300,
        }
    }
    pub const fn light_level(&self) -> f32 {
        match self {
            PlayerState::Idle => 0.225,
            PlayerState::Swimming => 0.175,
            PlayerState::FastSwimming => 0.1,
        }
    }
}
