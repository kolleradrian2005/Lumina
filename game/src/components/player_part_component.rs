use lumina_engine::logic::scene::ecs::component::component::Component;

#[derive(Component)]
pub enum PlayerPartComponent {
    LeftHand,
    Legs,
    Torso,
    RightHand,
    Tank,
    Head,
}
