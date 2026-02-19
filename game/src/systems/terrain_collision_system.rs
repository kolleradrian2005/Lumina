use std::sync::{Arc, Mutex};

use lumina_engine::scene::world::{
    component::{collider_component::ColliderComponent, transform_component::TransformComponent},
    system::system::System,
    world::World,
};

use crate::terrain::terrain::Terrain;

pub struct TerrainCollisionSystem;

impl System for TerrainCollisionSystem {
    fn run(&self, world: &mut World, _: f32) {
        let terrain = world.expect_resource::<Arc<Mutex<Terrain>>>().clone();
        for (_, (collider_component, transform)) in
            world.query_mut::<(&mut ColliderComponent, &mut TransformComponent)>()
        {
            let mut y_offset = 0.0;
            collider_component.collider.update(transform.clone());
            if let Ok(terrain) = terrain.lock() {
                for point in collider_component.collider.transformed_points {
                    let height = terrain.get_height(point.x);
                    if point.y < height {
                        y_offset = f32::max(y_offset, height - point.y);
                    }
                }
                transform.position.y += y_offset;
            }
        }
    }
}
