use lumina_engine::logic::{
    ecs::{
        component::{collider::Collider, transform::Transform},
        system::system::System,
    },
    scene::world::World,
};

use crate::scene::terrain::Terrain;

pub struct TerrainCollisionSystem;

impl System for TerrainCollisionSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (collider, transform)) in world.query_mut::<(&mut Collider, &mut Transform)>() {
            for point in &collider.boundary_points {
                let terrain = world.expect_resource::<Terrain>();
                let height = terrain.get_height(point.x);
                if point.y < height {
                    transform.position.y += height - point.y;
                }
            }
        }
    }
}
