use crate::{
    render::{render_entity::RenderEntity, window_size::WindowSize},
    scene::world::component::camera_component::CameraComponent,
};

#[derive(Clone, Debug)]
pub struct ExtractedFrame {
    pub entities: Vec<RenderEntity>,
    pub camera_component: Option<CameraComponent>, // Set this to refresh view matrices on resize
    pub window_size: Option<WindowSize>,
}
