use crate::engine::{math::{vec3::Vec3, vec2::Vec2, transformation}, render::updatable::Updatable, transformable::Transformable, window_handler::WindowHandler};

use super::player::Player;


pub struct Camera {
    position: Vec3,
    move_speed: f32,
    zoom_speed: f32,
    near: f32,
    far: f32,
    focal_offset: Vec2,
    max_distance_from_player: f32
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.25, 0.0),
            move_speed: 0.005,
            zoom_speed: 0.1,
            near: 0.0,
            far: 10.0,
            focal_offset: Vec2::new(0.0, 0.0),
            max_distance_from_player: 0.25
        }
    }
    
    pub fn update(&mut self, delta_time: f32, player: &mut Player, updatables: &mut Vec<Updatable>) {
        let player_position = player.model_group.get_position();
        let mut camera_position = self.get_position().clone();

        let x_max_dist = self.max_distance_from_player;
        let y_max_dist = self.max_distance_from_player;

        let treshold = 0.002;
        let mut difference = Vec3::from_vec2(player_position.xy() - camera_position.xy(), 0.0);
        if difference.x.abs() < treshold {
            difference.x = 0.0;
        }
        if difference.y.abs() < treshold {
            difference.y = 0.0;
        }
        let direction = difference.normalized();
        let length = difference.length();
        
        if 0.0 < length {
            updatables.push(Updatable::View)
        }

        let mut move_speed = self.move_speed.clone();
        
        if length < move_speed {
            move_speed = length;
        }

        camera_position += direction * move_speed * f32::sqrt(length) * delta_time as f32 * 100.0;
        
        camera_position.x = camera_position.x.clamp(
            player_position.x - x_max_dist,
            player_position.x + x_max_dist
        );

        camera_position.y = camera_position.y.clamp(
            player_position.y - y_max_dist,
            player_position.y + y_max_dist
        );

        let z_dest = player.get_state().zoom();
        let z_curr = self.get_position().z;
        let difference = z_dest - z_curr;
        let change = difference.signum() * (delta_time * self.zoom_speed);
        
        if (z_dest - z_curr).abs() < change {
            camera_position.z = z_dest;
        } else {
            camera_position.z = z_curr + change;
        }

        self.focal_offset = player_position.xy() - camera_position.xy();
        self.set_position(camera_position);
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_view_matrix(self.position)
    }
    pub fn get_projection_matrix(&self, window_handler: &WindowHandler) -> [[f32; 4]; 4] {
        transformation::create_ortographic_projection_matrix(window_handler.get_aspect_ratio(), self.near, self.far)
    }
    pub fn get_focal_offset(&self) -> &Vec2 {
        &self.focal_offset
    }
}
