extern crate gl;
extern crate glutin;
extern crate glutin_winit;

#[macro_use]
pub mod macros;

pub mod app;
pub(crate) mod engine_config;
pub mod logic;
pub mod math;
pub mod render;
pub mod shared;

pub use app::start;

#[cfg(target_os = "android")]
pub use app::start_with_android_app;
