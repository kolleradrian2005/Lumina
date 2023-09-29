use crate::scene::Scene;

pub struct Physics {}

impl Physics {
    pub fn new() -> Self {
        Physics { }
    }
    
    pub fn update(&self, scene: &mut Scene, delta_time: u128) {
        scene.update(delta_time);
    }
}