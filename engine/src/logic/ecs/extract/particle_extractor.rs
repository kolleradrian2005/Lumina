use crate::{
    logic::{
        ecs::{
            component::{emitter::Emitter, model::Model, transform::Transform},
            extract::{extractor::Extractor, model_extractor::ModelExtractor},
        },
        scene::world::World,
    },
    shared::extracted_frame::ExtractedFrame,
};
pub struct ParticleExtractor;

impl Extractor for ParticleExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        for (entity, (emitter, model, transform)) in world.query::<(&Emitter, &Model, &Transform)>()
        {
            for particle in emitter.particles.iter() {
                ModelExtractor::prepare_entity(
                    world,
                    frame,
                    entity,
                    None,
                    Transform {
                        position: particle.position,
                        rotation: transform.rotation,
                        scale: transform.scale,
                        is_flipped: transform.is_flipped,
                    },
                    model,
                );
            }
        }
    }
}
