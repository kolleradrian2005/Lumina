use super::component::Component;

pub enum PlayerPartComponent {
    LeftHand,
    Legs,
    Torso,
    RightHand,
    Tank,
    Head,
}

impl Component for PlayerPartComponent {}
