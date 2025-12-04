use lumina_engine::scene::world::{component::component::Component, entity::entity::Entity};

#[derive(Debug)]
pub enum AnimationCondition {
    None,
    PlayerIdle,
    PlayerSwimming,
    True,
}

#[derive(Component, Debug)]
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
