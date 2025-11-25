use crate::scene::{
    water::Water,
    world::{
        component::{
            movement_component::MovementComponent,
            shader_params_component::{ShaderParam, ShaderParamsComponent},
            transform_component::TransformComponent,
        },
        system::system::System,
        world::World,
    },
};

pub struct CurrentSystem;

impl System for CurrentSystem {
    fn run(&self, world: &mut World, _: f32) {
        for (_, (shader_params_component, transform_component)) in
            world.query_mut::<(&mut ShaderParamsComponent, &mut TransformComponent)>()
        {
            if shader_params_component.params.is_empty() {
                continue;
            }
            if let Some(ShaderParam::Current(current)) = shader_params_component.params.get_mut(0) {
                let object_position = transform_component.position;
                let mut water_current = world
                    .expect_resource::<Water>()
                    .get_current(&object_position);
                world
                    .query_mut::<(&mut MovementComponent, &mut TransformComponent)>()
                    .last()
                    .map(|(_, (movement_component, transform))| {
                        let player_distance = (object_position - transform.position).length();
                        if player_distance != 0.0 {
                            let mut influence = 1.0 / (player_distance.powf(1.5) * 10.0);
                            let influence_treshold = 5.5;
                            influence = f32::min(influence_treshold, influence);
                            water_current += influence * movement_component.velocity.x;
                        }
                    });
                *current = water_current;
            }
        }
    }
}
