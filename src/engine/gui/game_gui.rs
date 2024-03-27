use std::{cell::RefCell, rc::Rc};

use crate::engine::{
    model::model::Model, texture::resource_manager::ResourceManager, window_handler::WindowHandler,
};

use super::{
    elements::{align::Align, container::Container, gesture_detector::GestureDetector, text::Text},
    gui::{self, Gui},
    gui_element::{Alignment, EdgeInsets, UiElement},
    listener::Listener,
};

pub struct GameGui {
    //state: Box<UiElement>,
    elements: Vec<Model>,
    listeners: Vec<Listener>,
    counter: Rc<RefCell<i32>>,
}

impl GameGui {
    pub fn create() -> Self {
        GameGui {
            elements: Vec::new(),
            listeners: Vec::new(),
            counter: Rc::new(RefCell::new(0)),
        }
    }
}

impl Gui for GameGui {
    fn build(&mut self, resource_manager: &ResourceManager, window_handler: &WindowHandler) {
        let counter = self.counter.clone();
        let state: Box<UiElement> = Align {
            alignment: Alignment::Top,
            child: GestureDetector {
                on_click: Rc::new(RefCell::new(Box::new(move || {
                    let mut counter = counter.borrow_mut();
                    *counter += 1;
                    true
                }))),
                child: Container {
                    padding: EdgeInsets::Zero,
                    color: None,
                    child: Some(
                        Align {
                            alignment: Alignment::Center,
                            child: Text {
                                text: self.counter.borrow().to_string(), //"Lumina".into(), //num.to_string(),
                                font: resource_manager.get_font("default"),
                                font_size: 0.001,
                            }
                            .into(),
                        }
                        .into(),
                    ),
                    width: 0.4,
                    height: 0.15,
                }
                .into(),
            }
            .into(),
        }
        .into();
        (self.elements, self.listeners) = gui::build(&state, window_handler);
    }

    fn get_listeners(&mut self) -> &Vec<Listener> {
        &self.listeners
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<Listener> {
        &mut self.listeners
    }

    fn get_elements(&self) -> &Vec<Model> {
        &self.elements
    }

    fn get_elements_mut(&mut self) -> &mut Vec<Model> {
        &mut self.elements
    }
}
