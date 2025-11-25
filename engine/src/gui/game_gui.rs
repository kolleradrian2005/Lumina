use std::sync::{Arc, RwLock};

use crate::{model::model::Model, texture::resource_provider::ResourceProvider};

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
    counter: Arc<RwLock<i32>>,
}

impl GameGui {
    pub fn create() -> Self {
        GameGui {
            elements: Vec::new(),
            listeners: Vec::new(),
            counter: Arc::new(RwLock::new(0)),
        }
    }
}

impl Gui for GameGui {
    fn build(&mut self, resource_provider: &dyn ResourceProvider, aspect_ratio: f32) {
        let counter = self.counter.clone();
        let state: Box<UiElement> = Align {
            alignment: Alignment::Top,
            child: GestureDetector {
                on_click: Arc::new(RwLock::new(Box::new(move || {
                    let mut counter = counter.write().unwrap();
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
                                text: format!(
                                    "Counter: {}",
                                    self.counter.read().unwrap().to_string()
                                ), //"Lumina".into(), //num.to_string(),
                                font: resource_provider.get_font("default"),
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
        (self.elements, self.listeners) = gui::build(&state, aspect_ratio);
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
