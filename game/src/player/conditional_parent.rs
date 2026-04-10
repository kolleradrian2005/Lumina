use lumina_engine::logic::ecs::{component::component::Component, entity::entity::Entity};

#[derive(Debug)]
pub enum AnimationCondition {
    None,
    PlayerIdle,
    PlayerSwimming,
    True,
}

#[derive(Component, Debug)]
pub struct ConditionalParent {
    pub parent: Entity,
    pub condition: AnimationCondition,
}

impl From<Entity> for ConditionalParent {
    fn from(parent: Entity) -> Self {
        Self {
            parent,
            condition: AnimationCondition::None,
        }
    }
}
