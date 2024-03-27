use super::{listener::RawListener, ui_model::UiModel};

pub struct UiModelGroup {
    pub models: Vec<UiModel>,
    pub listeners: Vec<RawListener>,
    pub dimensions: (f32, f32),
}

impl UiModelGroup {
    pub fn new() -> UiModelGroup {
        Self {
            models: Vec::new(),
            listeners: Vec::new(),
            dimensions: (0.0, 0.0),
        }
    }

    pub fn add_margin(&mut self, margin: (f32, f32)) {
        for model in self.models.iter_mut() {
            model.margin.0 += margin.0;
            model.margin.1 += margin.1;
        }
        for listener in self.listeners.iter_mut() {
            listener.ui_model.margin.0 += margin.0;
            listener.ui_model.margin.1 += margin.1;
        }
    }
}

