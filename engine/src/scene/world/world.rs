use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    mem,
    sync::Arc,
};

use crate::{
    model::mesh::Mesh,
    render::render_packet::RenderPacket,
    scene::world::{
        component::model_component::ModelComponent,
        query::QueryMut,
    },
    texture::resource_manager::ResourceManager,
};

use super::{
    component::component::Component,
    component_storage::{ComponentStorage, ComponentStorageTrait},
    entity::{entity::Entity, particle_entity::ParticleEntity},
    query::Query,
};

extern crate noise;
pub struct World {
    pub entities: Vec<Entity>,
    available_ids: Vec<u32>,
    pub components: HashMap<TypeId, Box<dyn ComponentStorageTrait + Send + Sync>>,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    pub particle_pool: Vec<ParticleEntity>,
    pub render_packet: RenderPacket,
}

impl World {
    pub fn load() -> Self {
        World {
            entities: vec![Entity(0)], // Have 0 as a null entity
            available_ids: Vec::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
            particle_pool: Vec::new(),
            render_packet: RenderPacket {
                entities: Vec::new(),
                window_resize: None,
                camera_component: None,
            },
        }
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let storage = self
            .components
            .entry(TypeId::of::<T>())
            .or_insert(ComponentStorage::<T>::new());
        storage
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap()
            .add(entity, Box::new(component));
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?
            .get(entity)?
            .downcast_ref::<T>()
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) -> Option<T> {
        self.components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()?
            .remove(entity)
    }

    pub fn get_component_mut<T: Component>(&self, entity: Entity) -> Option<&mut T> {
        let storage = self
            .components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?;
        let ptr: *mut ComponentStorage<T> = storage as *const _ as *mut _;
        unsafe { ptr.as_mut()?.get_mut(entity)?.downcast_mut::<T>() }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self
            .available_ids
            .pop()
            .unwrap_or(self.entities.len() as u32);
        let entity = Entity(entity_id);
        self.entities.push(entity);
        entity
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        if let Some(idx) = self.entities.iter().position(|e| e.0 == entity.0) {
            if let Some(model_component) = self.remove_component::<ModelComponent>(entity) {
                self.mesh_removed(model_component.mesh);
            }
            self.entities.remove(idx);
            self.available_ids.push(entity.0);
            for storage in self.components.values_mut() {
                storage.remove_entity(entity);
            }
        }
    }

    pub fn insert_resource<T: 'static + Any + Send + Sync>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: 'static + Any + Send + Sync>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn expect_resource<T: 'static + Any + Send + Sync>(&self) -> &T {
        self.get_resource::<T>()
            .unwrap_or_else(|| panic!("Resource {:?} not found", type_name::<T>()))
    }

    pub fn expect_resource_mut<T: 'static + Any + Send + Sync>(&mut self) -> &mut T {
        self.get_resource_mut::<T>()
            .unwrap_or_else(|| panic!("Resource {:?} not found", type_name::<T>()))
    }

    pub fn get_resource_mut<T: 'static + Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    pub fn get_resource_ptr<T: 'static + Any + Send + Sync>(&mut self) -> Option<*mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
            .map(|resource| resource as *mut T)
    }

    pub fn expect_resource_ptr<T: 'static + Any + Send + Sync>(&mut self) -> *mut T {
        self.get_resource_ptr::<T>()
            .unwrap_or_else(|| panic!("Resource {:?} not found", type_name::<T>()))
    }

    pub fn get_storage<T: Component>(&self) -> Option<&ComponentStorage<T>> {
        self.components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()
    }

    pub fn get_storage_ptr<T: Component>(&self) -> Option<*const ComponentStorage<T>> {
        self.components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()
            .map(|storage| storage as *const ComponentStorage<T>)
    }

    pub fn get_storage_ptr_mut<T: Component>(&mut self) -> Option<*mut ComponentStorage<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .map(|storage| storage as *mut ComponentStorage<T>)
    }

    pub fn query_mut<'a, T: QueryMut<'a>>(&mut self) -> T::Iterator {
        T::create_query(self)
    }

    pub fn query<'a, T: Query<'a>>(&self) -> T::Iterator {
        T::create_query(self)
    }

    pub fn clear_render_packet(&mut self) {
        for renderable in mem::take(&mut self.render_packet.entities) {
            self.mesh_removed(renderable.mesh);
        }
        self.render_packet.window_resize = None;
        self.render_packet.camera_component = None;
    }

    fn mesh_removed(&mut self, mesh: Arc<Mesh>) {
        if let Some(mesh) = Arc::into_inner(mesh) {
            if let Some(resource_manager) = self.get_resource_mut::<ResourceManager>() {
                resource_manager.unload_mesh(mesh);
            }
        }
    }
}
