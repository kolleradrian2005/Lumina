use std::marker::PhantomData;

use crate::logic::scene::world::World;

use super::{
    component::component::Component, component_storage::ComponentStorage, entity::entity::Entity,
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

macro_rules! impl_query {
    ($IterName:ident, $(($Comp:ident => $var:ident)),+) => {
        pub struct $IterName<'a, $($Comp: Component),+> {
            entities_ptr: *const Entity,
            entities_len: usize,
            $( $var: Option<*const ComponentStorage<$Comp>>, ) +
            index: usize,
            #[allow(unused_parens)]
            _marker: PhantomData<&'a ( $($Comp),+ )>,
        }

        impl<'a, $($Comp: Component),+> Query<'a> for ( $(&'a $Comp),+, ) {
            type Item = ( $(&'a $Comp),+, );
            type Iterator = $IterName<'a, $($Comp),+>;

            fn create_query(world: &World) -> Self::Iterator {
                $IterName {
                    entities_ptr: world.entities.as_ptr(),
                    entities_len: world.entities.len(),
                    $( $var: world.get_storage_ptr::<$Comp>(), )+
                    index: 0,
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, $($Comp: Component),+> Iterator for $IterName<'a, $($Comp),+> {
            type Item = (Entity, ( $(&'a $Comp),+, ));

            fn next(&mut self) -> Option<Self::Item> {
                #[allow(unused_parens)]
                if let ( $( Some($var) ),+ ) = ( $( self.$var ),+ ) {
                    while self.index < self.entities_len {
                        let entity = unsafe { *self.entities_ptr.add(self.index) };
                        self.index += 1;

                        unsafe {
                            $( let $var = &* $var; )+

                            if let ( $( Some($var) ),+ ) = ( $( $var.get(entity) ),+ ) {
                                if let ( $( Some($var) ),+ ) = ( $( $var.downcast_ref() ),+ ) {
                                    return Some((entity, ( $( $var ),+, )));
                                }
                            }
                        }
                    }
                }
                None
            }
        }
    };
}

macro_rules! impl_query_mut {
    ($IterName:ident, $(($Comp:ident => $var:ident)),+) => {
        pub struct $IterName<'a, $($Comp: Component),+> {
            entities_ptr: *const Entity,
            entities_len: usize,
            $( $var: Option<*mut ComponentStorage<$Comp>>, ) +
            index: usize,
            #[allow(unused_parens)]
            _marker: PhantomData<&'a ( $($Comp),+ )>,
        }

        impl<'a, $($Comp: Component),+> QueryMut<'a> for ( $(&'a mut $Comp),+, ) {
            type Item = ( $(&'a mut $Comp),+, );
            type Iterator = $IterName<'a, $($Comp),+>;

            fn create_query(world: &mut World) -> Self::Iterator {
                $IterName {
                    entities_ptr: world.entities.as_ptr(),
                    entities_len: world.entities.len(),
                    $( $var: world.get_storage_ptr_mut::<$Comp>(), )+
                    index: 0,
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, $($Comp: Component),+> Iterator for $IterName<'a, $($Comp),+> {
            type Item = (Entity, ( $(&'a mut $Comp),+, ));

            fn next(&mut self) -> Option<Self::Item> {
                #[allow(unused_parens)]
                if let ( $( Some($var) ),+ ) = ( $( self.$var ),+ ) {
                    while self.index < self.entities_len {
                        let entity = unsafe { *self.entities_ptr.add(self.index) };
                        self.index += 1;

                        unsafe {
                            $( let $var = &mut * $var; )+

                            if let ( $( Some($var) ),+ ) = ( $( $var.get_mut(entity) ),+ ) {
                                if let ( $( Some($var) ),+ ) = ( $( $var.downcast_mut() ),+ ) {
                                    return Some((entity, ( $( $var ),+, )));
                                }
                            }
                        }
                    }
                }
                None
            }
        }
    };
}

// TODO: Handle single component case separately to avoid the tuple overhead
impl_query!(SingleComponentIter, (A => a));
impl_query!(DoubleComponentIter, (A => a), (B => b));
impl_query!(TripleComponentIter, (A => a), (B => b), (C => c));
impl_query!(FourComponentIter, (A => a), (B => b), (C => c), (D => d));
impl_query!(FiveComponentIter, (A => a), (B => b), (C => c), (D => d), (E => e));
impl_query!(SixComponentIter, (A => a), (B => b), (C => c), (D => d), (E => e), (F => f));
impl_query_mut!(SingleComponentIterMut, (A => a));
impl_query_mut!(DoubleComponentIterMut, (A => a), (B => b));
impl_query_mut!(TripleComponentIterMut, (A => a), (B => b), (C => c));
impl_query_mut!(FourComponentIterMut, (A => a), (B => b), (C => c), (D => d));
impl_query_mut!(FiveComponentIterMut, (A => a), (B => b), (C => c), (D => d), (E => e));
impl_query_mut!(SixComponentIterMut, (A => a), (B => b), (C => c), (D => d), (E => e), (F => f));
