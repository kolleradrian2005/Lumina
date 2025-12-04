use crate::{collider::Collider, scene::world::component::component::Component};

#[derive(Component, Clone)]
pub struct ColliderComponent {
    pub collider: Collider,
}

impl From<Collider> for ColliderComponent {
    fn from(collider: Collider) -> Self {
        Self { collider }
    }
}
