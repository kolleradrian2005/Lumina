use lumina_engine::logic::scene::ecs::component::component::Component;

#[derive(Component, Debug, Default)]
pub struct CurrentComponent {
    pub current: f32,
}
