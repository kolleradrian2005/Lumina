use lumina_engine::logic::ecs::component::component::Component;

#[derive(Component, Debug, Default)]
pub struct Current {
    pub current: f32,
}
