use rand::rngs::StdRng;

use crate::{
    focus_point::FocusPoint,
    scene::world::{
        component::{emitter_component::EmitterComponent, model_component::ModelComponent},
        entity::particle_entity::ParticleEntity,
        world::World,
    },
};

use super::system::System;

pub struct EmitterSystem;

impl System for EmitterSystem {
    fn run(&self, world: &mut World, delta_time: f32) {
        let mut removeables = vec![];
        let rng: *mut StdRng = world.expect_resource_ptr::<StdRng>();
        for (entity, (emitter, model)) in
            world.query_mut::<(&mut EmitterComponent, &mut ModelComponent)>()
        {
            emitter.cycle_time += delta_time;
            emitter.now += delta_time;
            let should_spawn = match emitter.lifespan {
                Some(sp) => emitter.now <= sp.as_secs_f32(),
                None => true,
            };
            let mut has_loaded = should_spawn;
            let focus_point = world.get_resource::<FocusPoint>();
            emitter.particles.retain_mut(|particle| {
                particle.update(delta_time);
                if !has_loaded {
                    if let Some(focus_point) = focus_point {
                        if particle.position.distance(focus_point.0)
                            <= emitter.cull_radius.unwrap_or(f32::INFINITY)
                        {
                            has_loaded = true;
                        }
                    }
                }
                particle.alive
            });
            if should_spawn {
                let count = emitter.cycle_time / emitter.interval.as_secs_f32();
                for i in 0..count as usize {
                    let mut particle = ParticleEntity::spawn(
                        emitter.emitter_type.clone(),
                        emitter.spawn_position,
                        model.clone(),
                        unsafe { &mut *rng },
                    );
                    particle.lifespan = emitter.particle_lifespan;

                    particle.set_speed(emitter.particle_velocity);
                    particle.update(i as f32 * emitter.interval.as_secs_f32());
                    emitter.particles.push(particle);
                }
                emitter.cycle_time -= count.floor() * emitter.interval.as_secs_f32();
            }
            if let Some(timeout) = &emitter.timeout {
                if emitter.now - timeout.start > timeout.duration {
                    emitter.particles.clear()
                }
            }
            if !has_loaded {
                emitter.particles.clear();
            }
            if !should_spawn && emitter.particles.is_empty() {
                emitter.alive = false;
                removeables.push(entity);
            }
        }
        for entity in removeables {
            world.delete_entity(entity);
        }
    }
}
