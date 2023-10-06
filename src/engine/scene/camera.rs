use crate::{vec2::Vec2, transformation, window_handler};

pub struct Camera {
    position: Vec2,
    move_speed: f32
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec2::new(0.0, 0.0),
            move_speed: 0.005
        }
    }
    
    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn change_position(&mut self, offset: &Vec2) {
        self.position = self.position.plus(offset);
    }

    pub fn get_move_speed(&self) -> &f32 {
        &self.move_speed
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_view_matrix(&self.position)
    }

    pub fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        let near = -1.0;
        let far = 1.0;
        // Todo: get real window sizes
        unsafe {
            transformation::create_ortographic_projection_matrix(window_handler::WINDOW_WIDTH, window_handler::WINDOW_HEIGHT, near, far)
        }
    }
}
