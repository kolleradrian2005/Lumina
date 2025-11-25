use crate::scene::world::component::camera_component::CameraComponent;

pub enum Updatable {
    Projection { width: i32, height: i32 },
    View { camera_component: CameraComponent },
    FocalRadius,
}
