use std::any::Any;

pub trait Component: 'static + Any + Send + Sync {}
