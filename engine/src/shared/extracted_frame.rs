use crate::{
    render::{uniformbuffer::UniformBufferData, window_size::WindowSize},
    shared::{postprocess_config::PostprocessConfig, render_entity::RenderEntity},
};

#[derive(Clone, Debug)]
pub struct ExtractedFrame {
    pub entities: Vec<RenderEntity>,
    pub uniform_buffers: Vec<UniformBufferData>,
    pub window_size: Option<WindowSize>,
    pub postprocess_pass: Option<PostprocessConfig>,
}
