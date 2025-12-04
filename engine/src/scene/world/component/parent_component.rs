use crate::scene::world::entity::entity::Entity;

use super::component::Component;

#[derive(Component, Clone)]
pub struct ParentComponent {
    pub parent: Entity,
}

impl From<Entity> for ParentComponent {
    fn from(parent: Entity) -> Self {
        Self { parent }
    }
}
