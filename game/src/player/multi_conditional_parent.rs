use lumina_engine::logic::ecs::component::component::Component;

use super::conditional_parent::ConditionalParent;

#[derive(Component, Debug)]
pub struct MultiConditionalParent {
    pub components: Vec<ConditionalParent>,
}

impl From<Vec<ConditionalParent>> for MultiConditionalParent {
    fn from(components: Vec<ConditionalParent>) -> Self {
        Self { components }
    }
}
