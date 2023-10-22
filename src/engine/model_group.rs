use crate::{model::Model, vec2::Vec2};

pub struct ModelGroup {
    models: Vec<Model>,
    // Transforming
    position: Vec2,
    rotation: f32,
    scale: f32
}

impl ModelGroup {
    pub fn new() -> Self {
        let models = Vec::new();
        ModelGroup {
            models: models,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            scale: 1.0
        }
    }
    
    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> &f32 {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn get_scale(&self) -> &f32 {
        &self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
    pub fn get_model(&mut self, index: usize) -> &mut Model {
        let model = self.models.get_mut(index);
        if model.is_none() {
            panic!("No such model at index: {}", index);
        }
        model.unwrap()
    }
    pub fn get_models(&mut self) -> &mut Vec<Model> {
        &mut self.models
    }

    pub fn set_flipped(&mut self, state: bool) {
        for model in self.models.iter_mut() {
            if model.is_flipped() != state {
                model.set_flipped(state);
                let mut flipped_position = model.get_position().clone();
                flipped_position.x = -flipped_position.x;
                model.set_position(flipped_position);
            }
        }
    }
}
