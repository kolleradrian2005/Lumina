use std::sync::Arc;

use crate::engine::{
    command_queue::CommandQueue,
    gui::{gui_element::GuiElement, ui_model_group::UiModelGroup},
    texture::font_texture::FontTexture,
};

pub struct Text {
    pub text: String,
    pub font: FontTexture,
    pub font_size: f32,
}

impl GuiElement for Text {
    fn collect_models(&self, command_queue: Arc<CommandQueue>, _: (f32, f32)) -> UiModelGroup {
        let mut model_group = UiModelGroup::new();
        let mut offset = 0.0;
        let mut max_height = 0.0;
        for c in self.text.chars() {
            let mut ui_model = self.font.get_model(command_queue.clone(), c);

            // Size
            let (ref mut width, ref mut height) = ui_model.size;
            *width *= self.font_size;
            *height *= self.font_size;

            // Margin
            let next_offset = ui_model.margin.0 * self.font_size;
            ui_model.margin.0 = offset;
            ui_model.margin.1 *= self.font_size;
            offset += next_offset + *width;

            max_height = f32::max(max_height, ui_model.margin.1 + *height);

            model_group.models.push(ui_model);
        }

        model_group.dimensions = (offset, max_height);
        model_group
    }
}
