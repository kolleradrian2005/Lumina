use crate::engine::gui::{
    gui_element::{Alignment, GuiElement, UiElement},
    ui_model_group::UiModelGroup,
};

pub struct Align {
    pub child: Box<UiElement>,
    pub alignment: Alignment,
}

impl GuiElement for Align {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup {
        let mut model_group = self.child.collect_models(max_size);
        let (width, height) = model_group.dimensions;
        let inv_width = max_size.0 - width;
        let inv_height = max_size.1 - height;
        let pos = match self.alignment {
            Alignment::Center => (inv_width / 2.0, inv_height / 2.0),
            Alignment::TopLeft => (0.0, 0.0),
            Alignment::Top => (inv_width / 2.0, 0.0),
            Alignment::TopRight => (inv_width, 0.0),
            Alignment::Right => (inv_width, inv_height / 2.0),
            Alignment::BottomRight => (inv_width, inv_height),
            Alignment::Bottom => (inv_width / 2.0, inv_height),
            Alignment::BottomLeft => (0.0, inv_height),
            Alignment::Left => (0.0, inv_height / 2.0),
        };
        model_group.add_margin(pos);
        model_group
    }
}
