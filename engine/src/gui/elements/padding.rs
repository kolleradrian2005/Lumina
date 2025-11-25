use crate::gui::{
    gui_element::{EdgeInsets, GuiElement, UiElement},
    ui_model_group::UiModelGroup,
};

impl EdgeInsets {
    pub fn get_attributes(&self) -> (f32, f32, f32, f32) {
        match self {
            EdgeInsets::Zero => (0.0, 0.0, 0.0, 0.0),
            EdgeInsets::Each {
                left,
                top,
                right,
                bottom,
            } => (left + right, top + bottom, *left, *top),
            EdgeInsets::All(p) => (2.0 * p, 2.0 * p, *p, *p),
            EdgeInsets::Symmetric {
                vertical,
                horizontal,
            } => (2.0 * horizontal, 2.0 * vertical, *horizontal, *vertical),
        }
    }
}

pub struct Padding {
    pub child: Box<UiElement>,
    pub padding: EdgeInsets,
}

impl GuiElement for Padding {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup {
        let mut model_group = self.child.collect_models(max_size);
        if let EdgeInsets::Zero = self.padding {
            return model_group;
        }
        let (x_size, y_size, x_pad, y_pad) = self.padding.get_attributes();
        model_group.dimensions.0 += x_size;
        model_group.dimensions.1 += y_size;
        model_group.add_margin((x_pad, y_pad));
        model_group
    }
}
