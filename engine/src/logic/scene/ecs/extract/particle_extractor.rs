use crate::{
    logic::scene::{
        ecs::{
            component::{
                emitter_component::EmitterComponent, model_component::ModelComponent,
                transform_component::TransformComponent,
            },
            extract::{extractor::Extractor, model_extractor::ModelExtractor},
        },
        world::World,
    },
    shared::extracted_frame::ExtractedFrame,
};
pub struct ParticleExtractor;

impl Extractor for ParticleExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        for (entity, (emitter, model, transform)) in
            world.query::<(&EmitterComponent, &ModelComponent, &TransformComponent)>()
        {
            for particle in emitter.particles.iter() {
                ModelExtractor::prepare_entity(
                    world,
                    frame,
                    entity,
                    None,
                    TransformComponent {
                        position: transform.position + particle.position,
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
