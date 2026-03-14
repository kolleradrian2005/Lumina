use crate::{
    logic::scene::{ecs::extract::extractor::Extractor, world::World},
    render::uniformbuffer::{PostProcessUniformBuffer, UniformBufferSource},
    shared::{extracted_frame::ExtractedFrame, postprocess_config::PostprocessConfig},
};
pub struct PostprocessExtractor;

impl Extractor for PostprocessExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        if let Some(source) = world.get_resource::<UniformBufferSource<PostProcessUniformBuffer>>()
        {
            frame.uniform_buffers.push(source.extract());
        }
        if let Some(postprocess_config) = world.get_resource::<PostprocessConfig>() {
            frame.postprocess_pass = Some(postprocess_config.clone());
        }
    }
}
