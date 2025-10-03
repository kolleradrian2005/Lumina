use super::component::Component;

#[derive(Debug, Default)]
pub struct CurrentComponent {
    pub current: f32,
}

impl Component for CurrentComponent {}
