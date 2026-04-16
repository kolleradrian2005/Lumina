use lumina_engine::logic::{
    ecs::{
        component::{material::Material, movement::Movement, transform::Transform},
        system::system::System,
    },
    scene::world::World,
};

use crate::{player::player_state::PlayerState, scene::water::Water};
pub struct CurrentSystem;

impl System for CurrentSystem {
    fn run(&mut self, world: &mut World, _: f32) {
        for (_, (material_component, transform_component)) in
            world.query_mut::<(&mut Material, &mut Transform)>()
        {
            if material_component.parameters.is_empty() {
                continue;
            }
            if material_component.get_param("uCurrent").is_some() {
                let object_position = transform_component.position;
                let mut water_current = world
                    .expect_resource::<Water>()
                    .get_current(&object_position);
                world
                    .query_mut::<(&mut Movement, &mut Transform, &mut PlayerState)>()
                    .last()
                    .map(|(_, (movement_component, transform, _))| {
                        let player_distance = (object_position - transform.position).length();
                        if player_distance != 0.0 {
                            let mut influence = 1.0 / (player_distance.powf(1.5) * 10.0);
                            let influence_treshold = 5.5;
                            influence = f32::min(influence_treshold, influence);
                            water_current += influence * movement_component.velocity.x;
                        }
                    });
                material_component.set_param("uCurrent", water_current);
            }
        }
    }
}
