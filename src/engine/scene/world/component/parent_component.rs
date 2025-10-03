use crate::engine::scene::world::entity::entity::Entity;

use super::component::Component;

#[derive(Clone)]
pub struct ParentComponent {
    pub parent: Entity,
}

impl From<Entity> for ParentComponent {
    fn from(parent: Entity) -> Self {
        Self { parent }
    }
}

impl Component for ParentComponent {}
