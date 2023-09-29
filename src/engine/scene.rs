use crate::model::Model;
use crate::camera::Camera;
use crate::player::Player;
use crate::texture_handler::TextureHandler;

pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
    pub player: Player
}

impl Scene {
    pub fn new(texture_handler: &mut TextureHandler) -> Self {
        let models: Vec<Model> = Vec::new();
        let camera: Camera = Camera::new();
        let player: Player = Player::new(texture_handler);
        Scene { models, camera, player }
    }
    
    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn update(&mut self, delta_time: u128) {
        self.update_camera(&delta_time);
    }

    pub fn update_camera(&mut self, delta_time: &u128) {
        let player_position = self.player.get_position();
        let mut camera_position = self.camera.get_position().clone();

        let x_max_dist = 0.25;
        let y_max_dist = 0.25;
        
        if x_max_dist <= player_position.x - camera_position.x {
            camera_position.x = player_position.x - x_max_dist
        } else if x_max_dist <= camera_position.x - player_position.x {
            camera_position.x = player_position.x + x_max_dist
        }
        
        if y_max_dist <= player_position.y - camera_position.y {
            camera_position.y = player_position.y - y_max_dist
        } else if y_max_dist <= camera_position.y - player_position.y {
            camera_position.y = player_position.y + y_max_dist
        } 
        
        let difference = player_position.minus(&camera_position);
        let direction = difference.normalized();
        let length = difference.length();

        let mut move_speed = self.camera.get_move_speed().clone();
        
        if length < move_speed {
            move_speed = length;
        }

        camera_position = camera_position.plus(&direction.times(move_speed * f32::sqrt(length) * *delta_time as f32 / 10.0));
        self.camera.set_position(camera_position);
    }

}