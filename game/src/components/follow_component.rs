use lumina_engine::scene::world::{component::component::Component, entity::entity::Entity};

#[derive(Component)]
pub struct FollowComponent {
    pub max_distance: f32,
    pub target_entity: Entity,
}
