#[derive(PartialEq)]
pub enum PlayerStateDefinition {
    Idle,
    Swimming,
    FastSwimming,
}

impl PlayerStateDefinition {
    pub const fn zoom(&self) -> f32 {
        match self {
            PlayerStateDefinition::Idle => 0.0,
            PlayerStateDefinition::Swimming => 0.02,
            PlayerStateDefinition::FastSwimming => 0.05,
        }
    }
    pub const fn acceleration(&self) -> f32 {
        match self {
            PlayerStateDefinition::Idle => 0.0,
            PlayerStateDefinition::Swimming => 1.0,
            PlayerStateDefinition::FastSwimming => 1.25,
        }
    }
    pub const fn is_swimming(&self) -> bool {
        match self {
            PlayerStateDefinition::Idle => false,
            PlayerStateDefinition::Swimming => true,
            PlayerStateDefinition::FastSwimming => true,
        }
    }
    pub const fn legs_animation_time(&self) -> u128 {
        match self {
            PlayerStateDefinition::Idle => 0,
            PlayerStateDefinition::Swimming => 350,
            PlayerStateDefinition::FastSwimming => 300,
        }
    }
    pub const fn light_level(&self) -> f32 {
        match self {
            PlayerStateDefinition::Idle => 0.225,
            PlayerStateDefinition::Swimming => 0.175,
            PlayerStateDefinition::FastSwimming => 0.1,
        }
    }
}
