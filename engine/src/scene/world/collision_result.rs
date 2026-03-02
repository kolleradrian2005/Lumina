use crate::{math::vec2::Vec2, scene::world::entity::entity::Entity};

pub struct CollisionResult {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub collision_normal: Vec2,
    pub penetration_depth: f32,
}
