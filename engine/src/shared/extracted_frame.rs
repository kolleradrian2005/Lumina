use crate::shared::{
    postprocess_config::PostprocessConfig, render_entity::RenderEntity,
    uniform_buffer_render_data::UniformBufferRenderData, window_size::WindowSize,
};

#[derive(Clone, Debug)]
pub struct ExtractedFrame {
    pub entities: Vec<RenderEntity>,
    pub uniform_buffers: Vec<UniformBufferRenderData>,
    pub window_size: Option<WindowSize>,
    pub postprocess_pass: Option<PostprocessConfig>,
}
