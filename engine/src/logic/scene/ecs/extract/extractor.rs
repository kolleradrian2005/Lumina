use crate::{logic::scene::world::World, shared::extracted_frame::ExtractedFrame};

pub trait Extractor: Send + Sync {
    fn extract(&mut self, world: &World, frame: &mut ExtractedFrame);
}
