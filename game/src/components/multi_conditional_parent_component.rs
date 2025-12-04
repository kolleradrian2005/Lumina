use lumina_engine::scene::world::component::component::Component;

use super::conditional_parent_component::ConditionalParentComponent;

#[derive(Component, Debug)]
pub struct MultiConditionalParentComponent {
    pub components: Vec<ConditionalParentComponent>,
}

impl From<Vec<ConditionalParentComponent>> for MultiConditionalParentComponent {
    fn from(components: Vec<ConditionalParentComponent>) -> Self {
        Self { components }
    }
}
