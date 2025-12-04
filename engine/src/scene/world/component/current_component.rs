use crate::scene::world::component::component::Component;

#[derive(Component, Debug, Default)]
pub struct CurrentComponent {
    pub current: f32,
}
