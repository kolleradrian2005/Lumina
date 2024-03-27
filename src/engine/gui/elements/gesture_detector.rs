use std::{cell::RefCell, rc::Rc};

use crate::engine::{
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
    pub on_click: Rc<RefCell<Box<dyn FnMut() -> bool>>>,
}

impl GuiElement for GestureDetector {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup {
        let mut model_group = self.child.collect_models(max_size);
        model_group.listeners.push(RawListener {
            ui_model: UiModel {
                model: sprite::square(1.0),
                size: (model_group.dimensions.0, model_group.dimensions.1),
                margin: (0.0, 0.0),
            },
            fun: Rc::clone(&self.on_click),
        });
        model_group
    }
}
