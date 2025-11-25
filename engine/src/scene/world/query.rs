use std::marker::PhantomData;

use super::{
    component::component::Component, component_storage::ComponentStorage, entity::entity::Entity,
    world::World,
};

pub trait Query<'a> {
    type Item;
    type Iterator: Iterator<Item = (Entity, Self::Item)>;

    fn create_query(world: &World) -> Self::Iterator;
}

pub trait QueryMut<'a> {
    type Item;
    type Iterator: Iterator<Item = (Entity, Self::Item)>;

    fn create_query(world: &mut World) -> Self::Iterator;
}

// (&A)
impl<'a, A: Component> Query<'a> for &'a A {
    type Item = &'a A;
    type Iterator = SingleComponentIter<'a, A>;

    fn create_query(world: &World) -> Self::Iterator {
        let storage = world.get_storage_ptr::<A>();

        SingleComponentIter {
            entities: world.entities.clone(),
            storage,
            index: 0,
            _marker: PhantomData,
        }
    }
}
pub struct SingleComponentIter<'a, A: Component> {
    entities: Vec<Entity>,
    storage: Option<*const ComponentStorage<A>>,
    index: usize,
    _marker: PhantomData<&'a A>,
}
impl<'a, A: Component> Iterator for SingleComponentIter<'a, A> {
    type Item = (Entity, &'a A);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(storage) = self.storage {
            while self.index < self.entities.len() {
                let entity = self.entities[self.index];
                self.index += 1;
                unsafe {
                    let storage = &*storage;
                    if let Some(a) = storage.get(entity) {
                        if let Some(a) = a.downcast_ref() {
                            return Some((entity, a));
                        }
                    }
                }
            }
        }
        None
    }
}

// (&mut A)
impl<'a, A: Component> QueryMut<'a> for &'a mut A {
    type Item = &'a mut A;
    type Iterator = SingleComponentIterMut<'a, A>;

    fn create_query(world: &mut World) -> Self::Iterator {
        let storage = world.get_storage_ptr_mut::<A>();

        SingleComponentIterMut {
            entities: world.entities.clone(),
            storage,
            index: 0,
            _marker: PhantomData,
        }
    }
}
pub struct SingleComponentIterMut<'a, A: Component> {
    entities: Vec<Entity>,
    storage: Option<*mut ComponentStorage<A>>,
    index: usize,
    _marker: PhantomData<&'a A>,
}
impl<'a, A: Component> Iterator for SingleComponentIterMut<'a, A> {
    type Item = (Entity, &'a mut A);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(storage) = self.storage {
            while self.index < self.entities.len() {
                let entity = self.entities[self.index];
                self.index += 1;
                unsafe {
                    let storage = &mut *storage;
                    if let Some(a) = storage.get_mut(entity) {
                        if let Some(a) = a.downcast_mut() {
                            return Some((entity, a));
                        }
                    }
                }
            }
        }
        None
    }
}

// (&mut A, &mut B)

impl<'a, A: Component, B: Component> QueryMut<'a> for (&'a mut A, &'a mut B) {
    type Item = (&'a mut A, &'a mut B);
    type Iterator = DoubleComponentIterMut<'a, A, B>;

    fn create_query(world: &mut World) -> Self::Iterator {
        let storage_a = world.get_storage_ptr_mut::<A>();
        let storage_b = world.get_storage_ptr_mut::<B>();
        DoubleComponentIterMut {
            entities: world.entities.clone(),
            storage_a,
            storage_b,
            index: 0,
            _marker: PhantomData,
        }
    }
}
pub struct DoubleComponentIterMut<'a, A: Component, B: Component> {
    entities: Vec<Entity>,
    storage_a: Option<*mut ComponentStorage<A>>,
    storage_b: Option<*mut ComponentStorage<B>>,
    index: usize,
    _marker: PhantomData<&'a (A, B)>,
}
impl<'a, A: Component, B: Component> Iterator for DoubleComponentIterMut<'a, A, B> {
    type Item = (Entity, (&'a mut A, &'a mut B));

    fn next(&mut self) -> Option<Self::Item> {
        if let (
            Some(storage_a), //
            Some(storage_b), //
        ) = (
            self.storage_a, //
            self.storage_b, //
        ) {
            while self.index < self.entities.len() {
                let entity = self.entities[self.index];
                self.index += 1;

                unsafe {
                    let storage_a = &mut *storage_a; //
                    let storage_b = &mut *storage_b; //

                    if let (
                        Some(a), //
                        Some(b), //
                    ) = (
                        storage_a.get_mut(entity), //
                        storage_b.get_mut(entity), //
                    ) {
                        if let (
                            Some(a), //
                            Some(b), //
                        ) = (
                            a.downcast_mut(), //
                            b.downcast_mut(), //
                        ) {
                            return Some((entity, (a, b)));
                        }
                    }
                }
            }
        }
        None
    }
}

// (&mut A, &mut B, &mut C)
impl<'a, A: Component, B: Component, C: Component> QueryMut<'a>
    for (&'a mut A, &'a mut B, &'a mut C)
{
    type Item = (&'a mut A, &'a mut B, &'a mut C);
    type Iterator = TripleComponentIterMut<'a, A, B, C>;

    fn create_query(world: &mut World) -> Self::Iterator {
        let storage_a = world.get_storage_ptr_mut::<A>();
        let storage_b = world.get_storage_ptr_mut::<B>();
        let storage_c = world.get_storage_ptr_mut::<C>();
        TripleComponentIterMut {
            entities: world.entities.clone(),
            storage_a,
            storage_b,
            storage_c,
            index: 0,
            _marker: PhantomData,
        }
    }
}
pub struct TripleComponentIterMut<'a, A: Component, B: Component, C: Component> {
    entities: Vec<Entity>,
    storage_a: Option<*mut ComponentStorage<A>>,
    storage_b: Option<*mut ComponentStorage<B>>,
    storage_c: Option<*mut ComponentStorage<C>>,
    index: usize,
    _marker: PhantomData<&'a (A, B, C)>,
}
impl<'a, A: Component, B: Component, C: Component> Iterator
    for TripleComponentIterMut<'a, A, B, C>
{
    type Item = (Entity, (&'a mut A, &'a mut B, &'a mut C));

    fn next(&mut self) -> Option<Self::Item> {
        if let (
            Some(storage_a), //
            Some(storage_b), //
            Some(storage_c), //
        ) = (
            self.storage_a, //
            self.storage_b, //
            self.storage_c, //
        ) {
            while self.index < self.entities.len() {
                let entity = self.entities[self.index];
                self.index += 1;

                unsafe {
                    let storage_a = &mut *storage_a; //
                    let storage_b = &mut *storage_b; //
                    let storage_c = &mut *storage_c; //

                    if let (
                        Some(a), //
                        Some(b), //
                        Some(c), //
                    ) = (
                        storage_a.get_mut(entity), //
                        storage_b.get_mut(entity), //
                        storage_c.get_mut(entity), //
                    ) {
                        if let (
                            Some(a), //
                            Some(b), //
                            Some(c), //
                        ) = (
                            a.downcast_mut(), //
                            b.downcast_mut(), //
                            c.downcast_mut(), //
                        ) {
                            return Some((entity, (a, b, c)));
                        }
                    }
                }
            }
        }
        None
    }
}
