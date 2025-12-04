use std::any::Any;

pub trait Component: 'static + Any + Send + Sync {}

use lumina_macro;
pub use lumina_macro::Component;
