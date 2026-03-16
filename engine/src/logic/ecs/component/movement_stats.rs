use crate::{logic::ecs::component::component::Component, math::vec3::Vec3};

#[derive(Component, Default)]
pub struct MovementStats {
    pub direction: Vec3,
    pub acceleration: f32,
}
