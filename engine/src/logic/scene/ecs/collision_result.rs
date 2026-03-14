use crate::{logic::scene::ecs::entity::entity::Entity, math::vec2::Vec2};

pub struct CollisionResult {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub collision_normal: Vec2,
    pub penetration_depth: f32,
}
