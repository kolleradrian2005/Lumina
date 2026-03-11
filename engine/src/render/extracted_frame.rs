use crate::render::{
    postprocess_config::PostprocessConfig, render_entity::RenderEntity,
    uniformbuffer::UniformBufferData, window_size::WindowSize,
};

#[derive(Clone, Debug)]
pub struct ExtractedFrame {
    pub entities: Vec<RenderEntity>,
    //pub camera_component: Option<CameraComponent>, // Set this to refresh view matrices on resize
    pub uniform_buffers: Vec<UniformBufferData>,
    pub window_size: Option<WindowSize>,
    pub postprocess_pass: Option<PostprocessConfig>,
}
