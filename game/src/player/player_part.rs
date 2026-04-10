use lumina_engine::logic::ecs::component::component::Component;

#[derive(Component)]
pub enum PlayerPart {
    LeftHand,
    Legs,
    Torso,
    RightHand,
    Tank,
    Head,
}
