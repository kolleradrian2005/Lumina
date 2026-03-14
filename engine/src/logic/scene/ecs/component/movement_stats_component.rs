use crate::{logic::scene::ecs::component::component::Component, math::vec3::Vec3};

#[derive(Component, Default)]
pub struct MovementStatsComponent {
    pub direction: Vec3,
    pub acceleration: f32,
}
