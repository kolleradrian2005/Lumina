use lumina_engine::logic::ecs::{component::component::Component, entity::entity::Entity};

#[derive(Component)]
pub struct Follow {
    pub max_distance: f32,
    pub target_entity: Entity,
}
