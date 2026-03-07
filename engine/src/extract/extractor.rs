use crate::{render::extracted_frame::ExtractedFrame, scene::world::world::World};

pub trait Extractor: Send + Sync {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame);
}
