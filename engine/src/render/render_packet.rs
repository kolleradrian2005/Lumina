use crate::{
    render::render_entity::RenderEntity, scene::world::component::camera_component::CameraComponent,
};

#[derive(Clone, Debug)]
pub struct RenderPacket {
    pub entities: Vec<RenderEntity>,
    pub camera_component: Option<CameraComponent>, // Set this to refresh view matrices on resize
    pub window_resize: Option<(i32, i32)>,         // camera_component must be set if this is set
}
