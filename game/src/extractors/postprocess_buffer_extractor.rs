use lumina_engine::{
    logic::{ecs::extract::extractor::Extractor, scene::world::World},
    render::uniform_buffer_source::UniformBufferSource,
    shared::extracted_frame::ExtractedFrame,
};

use crate::postprocess_uniform_buffer::PostProcessUniformBuffer;

pub struct PostprocessBufferExtractor;

impl Extractor for PostprocessBufferExtractor {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame) {
        if let Some(source) = world.get_resource::<UniformBufferSource<PostProcessUniformBuffer>>()
        {
            frame.uniform_buffers.push(source.extract());
        }
    }
}
