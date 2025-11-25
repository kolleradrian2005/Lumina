

use super::{component::Component, conditional_parent_component::ConditionalParentComponent};

#[derive(Debug)]
pub struct MultiConditionalParentComponent {
    pub components: Vec<ConditionalParentComponent>,
}

impl From<Vec<ConditionalParentComponent>> for MultiConditionalParentComponent {
    fn from(components: Vec<ConditionalParentComponent>) -> Self {
        Self { components }
    }
}

impl Component for MultiConditionalParentComponent {}
