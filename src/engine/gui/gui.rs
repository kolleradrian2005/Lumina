

use crate::engine::{model::model::Model, texture::resource_provider::ResourceProvider};

use super::{
    gui_element::{GuiElement, UiElement},
    listener::{Listener, RawListener},
    ui_model::UiModel,
};

pub trait Gui {
    fn build(&mut self, resource_provider: &dyn ResourceProvider, aspect_ratio: f32);
    fn get_listeners(&mut self) -> &Vec<Listener>;
    fn get_listeners_mut(&mut self) -> &mut Vec<Listener>;
    fn get_elements(&self) -> &Vec<Model>;
    fn get_elements_mut(&mut self) -> &mut Vec<Model>;
}

pub fn build(state: &Box<UiElement>, aspect_ratio: f32) -> (Vec<Model>, Vec<Listener>) {
    let model_group = state.collect_models((2.0, 2.0 / aspect_ratio));
    (
        model_group
            .models
            .into_iter()
            .map(|model| UiModel::bake_model(model, aspect_ratio))
            .collect(),
        model_group
            .listeners
            .into_iter()
            .map(|listener| RawListener::bake_listener(listener, aspect_ratio))
            .collect(),
    )
}
