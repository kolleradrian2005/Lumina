use crate::{math::vec3::Vec3, scene::world::component::component::Component};

#[derive(Component, Default)]
pub struct MovementStatsComponent {
    pub direction: Vec3,
    pub acceleration: f32,
}
