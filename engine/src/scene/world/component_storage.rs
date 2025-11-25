use std::{any::Any, collections::HashMap};

use super::{component::component::Component, entity::entity::Entity};

pub struct ComponentStorage<T: Component> {
    storage: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> Box<ComponentStorage<T>> {
        Box::new(Self {
            storage: HashMap::new(),
        })
    }
    pub fn as_any(component: &Box<dyn Component>) -> &dyn Any {
        component
    }

    pub fn add(&mut self, entity: Entity, component: Box<dyn Any>) {
        if let Ok(comp) = component.downcast::<T>() {
            self.storage.insert(entity, *comp);
        } else {
            panic!("Attempted to insert a component of the wrong type")
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        self.storage.remove(&entity)
    }

    pub fn get(&self, entity: Entity) -> Option<&dyn Any> {
        self.storage.get(&entity).map(|c| c as &dyn Any)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut dyn Any> {
        self.storage.get_mut(&entity).map(|c| c as &mut dyn Any)
    }
}

pub trait ComponentStorageTrait: Any + Send + Sync + 'static {
    fn remove_entity(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> ComponentStorageTrait for ComponentStorage<T> {
    fn remove_entity(&mut self, entity: Entity) {
        self.storage.remove(&entity);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
