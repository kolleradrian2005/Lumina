use std::sync::{Arc, RwLock};

use crate::engine::{
    command_queue::CommandQueue,
    gui::{
        gui_element::{GuiElement, UiElement},
        listener::RawListener,
        ui_model::UiModel,
        ui_model_group::UiModelGroup,
    },
    model::sprite,
};

pub struct GestureDetector {
    pub child: Box<UiElement>,
    pub on_click: Arc<RwLock<Box<dyn FnMut() -> bool + Send + Sync>>>,
}

impl GuiElement for GestureDetector {
    fn collect_models(
        &self,
        command_queue: Arc<CommandQueue>,
        max_size: (f32, f32),
    ) -> UiModelGroup {
        let mut model_group = self.child.collect_models(command_queue.clone(), max_size);
        model_group.listeners.push(RawListener {
            ui_model: UiModel {
                model: sprite::square(command_queue.clone(), 1.0),
                size: (model_group.dimensions.0, model_group.dimensions.1),
                margin: (0.0, 0.0),
            },
            fun: Arc::clone(&self.on_click),
        });
        model_group
    }
}
