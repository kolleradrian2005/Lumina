use crate::{
    logic::{ecs::extract::extractor::Extractor, scene::world::World},
    shared::{extracted_frame::ExtractedFrame, postprocess_config::PostprocessConfig},
};
pub struct PostprocessExtractor;

impl Extractor for PostprocessExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        if let Some(postprocess_config) = world.get_resource::<PostprocessConfig>() {
            frame.postprocess_pass = Some(postprocess_config.clone());
        }
    }
}
