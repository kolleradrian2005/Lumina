use lumina_engine::scene::world::component::component::Component;

#[derive(Component)]
pub enum PlayerPartComponent {
    LeftHand,
    Legs,
    Torso,
    RightHand,
    Tank,
    Head,
}
