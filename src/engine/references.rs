// Window
pub const WINDOW_TITLE: &str = "Lumina";
pub const INITIAL_WINDOW_WIDTH: i32 = 1200;
pub const INITIAL_WINDOW_HEIGHT: i32 = 800;
// Assets

pub const TEXTURES_PATH: &str = "textures";
pub const FONTS_PATH: &str = "fonts";
//#[cfg(target_os = "android")]
//pub const SHADERS_PATH: &str = "es_shaders";
//#[cfg(not(target_os = "android"))]
pub const SHADERS_PATH: &str = "shaders";

#[cfg(target_os = "android")]
pub const SHADER_VERSION_HEADER: &str = "#version 300 es\r\n";
#[cfg(not(target_os = "android"))]
pub const SHADER_VERSION_HEADER: &str = "#version 450 core\r\n";
