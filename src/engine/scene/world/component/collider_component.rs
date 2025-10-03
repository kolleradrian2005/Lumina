use crate::engine::collider::Collider;

use super::component::Component;

#[derive(Clone)]
pub struct ColliderComponent {
    pub collider: Collider,
}

impl From<Collider> for ColliderComponent {
    fn from(collider: Collider) -> Self {
        Self { collider }
    }
}

impl Component for ColliderComponent {}
