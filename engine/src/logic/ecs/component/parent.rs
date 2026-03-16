use crate::logic::ecs::entity::entity::Entity;

use super::component::Component;

#[derive(Component, Clone)]
pub struct Parent {
    pub parent: Entity,
}

impl From<Entity> for Parent {
    fn from(parent: Entity) -> Self {
        Self { parent }
    }
}
