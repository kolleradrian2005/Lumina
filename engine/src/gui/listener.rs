use std::sync::{Arc, RwLock};

use crate::math::vec2::Vec2;

use super::ui_model::UiModel;

pub struct RawListener {
    pub ui_model: UiModel,
    pub fun: Arc<RwLock<Box<dyn FnMut() -> bool + Send + Sync>>>,
}

pub struct Listener {
    pub bottom_left: Vec2,
    pub top_right: Vec2,
    pub fun: Arc<RwLock<Box<dyn FnMut() -> bool + Send + Sync>>>,
}

impl RawListener {
    pub fn bake_listener(self, aspect_ratio: f32) -> Listener {
        Listener {
            bottom_left: (
                self.ui_model.margin.0,
                (self.ui_model.margin.1 + self.ui_model.size.1) * aspect_ratio,
            )
                .into(),
            top_right: (
                self.ui_model.margin.0 + self.ui_model.size.0,
                self.ui_model.margin.1 * aspect_ratio,
            )
                .into(),
            fun: self.fun,
        }
    }
}
