use crate::engine::{collider::Collider, math::{vec2::Vec2, vec3::Vec3}, transformable::Transformable};

use super::model::Model;

pub struct ModelGroup {
    models: Vec<Model>,
    // Transforming
    position: Vec3,
    rotation: f32,
    scale: Vec2,
    collider: Option<Collider>,
}

impl ModelGroup {
    pub fn new(collider: Option<Collider>) -> Self {
        ModelGroup {
            models: Vec::new(),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            scale: Vec2::uniform(1.0),
            collider,
        }
    }

    pub fn get_collider(&self) -> &Option<Collider> {
        &self.collider
    }

    pub fn replace_model(&mut self, index: usize, model: Model) {
        if let Some(dest_model) = self.get_model_mut(index) {
            let _ = std::mem::replace(dest_model, model);
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
    
    pub fn get_model_mut(&mut self, index: usize) -> Option<&mut Model> {
        self.models.get_mut(index)
    }

    pub fn get_model(&mut self, index: usize) -> Option<&Model> {
        self.models.get(index)
    }

    pub fn get_models_mut(&mut self) -> &mut Vec<Model> {
        &mut self.models
    }

    pub fn get_models(&self) -> &Vec<Model> {
        &self.models
    }

    pub fn set_flipped(&mut self, state: bool) {
        for model in self.models.iter_mut() {
            if model.is_flipped() != state {
                let mut flipped_position = model.get_position();
                flipped_position.x *= -1.0;
                model.set_position(flipped_position);
            }
            model.set_flipped(state);
        }
        if let Some(collider) = &mut self.collider {
            collider.set_flipped(state);
        }
    }
}

impl Transformable for ModelGroup {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        if let Some(collider) = &mut self.collider {
            collider.set_position(pos.xy());
        }
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
        if let Some(collider) = &mut self.collider {
            collider.set_rotation(rot);
        }
    }

    fn get_scale(&self) -> Vec2 {
        self.scale
    }

    fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        if let Some(collider) = &mut self.collider {
            collider.set_scale(scale);
        }
    }
}
