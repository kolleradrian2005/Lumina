use crate::scene::world::entity::entity::Entity;

use super::component::Component;

#[derive(Debug)]
pub enum AnimationCondition {
    None,
    PlayerIdle,
    PlayerSwimming,
    True,
}

#[derive(Debug)]
pub struct ConditionalParentComponent {
    pub parent: Entity,
    pub condition: AnimationCondition,
}

impl From<Entity> for ConditionalParentComponent {
    fn from(parent: Entity) -> Self {
        Self {
            parent,
            condition: AnimationCondition::None,
        }
    }
}

impl Component for ConditionalParentComponent {}
