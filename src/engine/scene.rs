use crate::model::Model;

pub struct Scene {
    models: Vec<Model>
}

impl Scene {
    pub fn new() -> Self {
        let models: Vec<Model> = Vec::new();
        Scene { models }
    }
    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
    pub fn get_models(&self) -> &Vec<Model> {
        &self.models
    }
}