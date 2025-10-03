

use crate::engine::{
    gui::{
        gui_element::{EdgeInsets, GuiElement, UiElement},
        ui_model::UiModel,
        ui_model_group::UiModelGroup,
    },
    model::sprite,
    texture::texture::StaticColor,
};

pub struct Container {
    pub child: Option<Box<UiElement>>,
    pub color: Option<StaticColor>,
    pub padding: EdgeInsets,
    pub width: f32,
    pub height: f32,
}

impl GuiElement for Container {
    fn collect_models(&self, _: (f32, f32)) -> UiModelGroup {
        let mut model_group = UiModelGroup::new();
        model_group.dimensions = (self.width, self.height);
        if let Some(color) = self.color {
            let mut model = sprite::square(1.0);
            model.set_texture(color.into());
            model_group.models.push(UiModel {
                model,
                size: (self.width, self.height),
                margin: (0.0, 0.0),
            });
        }
        if let Some(child) = &self.child {
            let mut child_model_group = child.collect_models((self.width, self.height));
            let (x_size, y_size, x_pad, y_pad) = self.padding.get_attributes();
            let (mut min_width, mut min_height) = child_model_group.dimensions;
            min_width += x_size;
            min_height += y_size;
            model_group.dimensions = (
                f32::max(self.width, min_width),
                f32::max(self.height, min_height),
            );

            child_model_group.add_margin((x_pad, y_pad));
            model_group.listeners.extend(child_model_group.listeners);
            model_group.models.extend(child_model_group.models);
        }
        model_group
    }
}
