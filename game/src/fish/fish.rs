use lumina_engine::logic::ecs::component::component::Component;

#[derive(Component, Clone)]
pub struct Fish {
    pub speed: f32,
}
