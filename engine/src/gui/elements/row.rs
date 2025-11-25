use crate::gui::{
    gui_element::{CrossAxisAlignment, GuiElement, MainAxisAlignment, UiElement},
    ui_model_group::UiModelGroup,
};

pub struct Row {
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub children: Vec<Box<UiElement>>,
}

impl GuiElement for Row {
    fn collect_models(&self, max_size: (f32, f32)) -> UiModelGroup {
        let children_count = self.children.len();
        let child_max_size = (max_size.0 / children_count as f32, max_size.1);

        let child_model_groups: Vec<_> = self
            .children
            .iter()
            .map(|child| child.collect_models(child_max_size))
            .collect();

        // Only calculate if necessary
        fn leftover(child_model_groups: &Vec<UiModelGroup>, max_size: (f32, f32)) -> f32 {
            max_size.0
                - child_model_groups
                    .iter()
                    .map(|model_group| model_group.dimensions.0)
                    .fold(0.0, |acc, val| acc + val)
        }

        let space = match self.main_axis_alignment {
            MainAxisAlignment::SpaceBetween => {
                leftover(&child_model_groups, max_size) / (children_count - 1) as f32
            }
            MainAxisAlignment::SpaceAround => {
                leftover(&child_model_groups, max_size) / children_count as f32
            }
            MainAxisAlignment::SpaceEvenly => {
                leftover(&child_model_groups, max_size) / (children_count + 1) as f32
            }
            _ => 0.0,
        };

        let mut x_offset = match self.main_axis_alignment {
            MainAxisAlignment::End => leftover(&child_model_groups, max_size),
            MainAxisAlignment::Center => leftover(&child_model_groups, max_size) / 2.0,
            MainAxisAlignment::SpaceAround => space / 2.0,
            MainAxisAlignment::SpaceEvenly => space,
            _ => 0.0,
        };

        let mut model_group = UiModelGroup::new();
        for mut child_model_group in child_model_groups {
            child_model_group.add_margin((
                x_offset,
                match self.cross_axis_alignment {
                    CrossAxisAlignment::Start => 0.0,
                    CrossAxisAlignment::End => max_size.1 - child_model_group.dimensions.1,
                    CrossAxisAlignment::Center => {
                        (max_size.1 - child_model_group.dimensions.1) / 2.0
                    }
                },
            ));

            model_group.listeners.extend(child_model_group.listeners);
            model_group.models.extend(child_model_group.models);

            x_offset += child_model_group.dimensions.0 + space;
        }
        model_group.dimensions = max_size;
        model_group
    }
}
