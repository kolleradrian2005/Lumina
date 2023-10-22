use std::f32::consts::PI;

use crate::background::Background;
use crate::camera::Camera;
use crate::foreground::Foreground;
use crate::model::Model;
use crate::player::Player;
use crate::terrain::Terrain;
use crate::texture_handler::TextureHandler;
use crate::vec2::Vec2;

pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
    pub player: Player,
    pub background: Background,
    pub foreground: Foreground,
    pub terrain: Terrain,
    pub focal_offset: Vec2,
}

impl Scene {
    pub fn new(texture_handler: &mut TextureHandler) -> Self {
        let models: Vec<Model> = Vec::new();
        let camera: Camera = Camera::new();
        let player: Player = Player::new(texture_handler);
        let background: Background = Background::construct(texture_handler);
        let foreground: Foreground = Foreground::construct();
        let focal_offset: Vec2 = Vec2::new(0.0, 0.0);
        let terrain = Terrain::new(696);
        Scene {
            models,
            camera,
            player,
            background,
            foreground,
            focal_offset,
            terrain,
        }
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn update(&mut self, delta_time: u128) {
        self.update_camera(&delta_time);
        self.update_player(&delta_time);
    }

    pub fn update_player(&mut self, delta_time: &u128) {
        // Load terrain correctly
        let tile_index = (self.player.get_position().x / self.terrain.tile_size).round() as i32;
        self.terrain.update_tile_index(tile_index);
        // Rotation animation
        let mut rotation = self.player.model_group.get_rotation().clone();
        let mut dest_rotation = self.player.get_dest_rotation();
        if dest_rotation < 0.0 {
            dest_rotation += 2.0 * PI;
        }
        dest_rotation %= 2.0 * PI;
        let mut difference = dest_rotation - rotation;
        if PI < difference {
            difference =  difference - 2.0 * PI;
        } else if PI < -difference {
            difference =  difference + 2.0 * PI;
        }
        let rot_speed = 0.005;
        rotation += rot_speed * difference * *delta_time as f32;
        rotation %= 2.0 * PI;
        if rotation < 0.0 {
            rotation += 2.0 * PI;
        }
        let head_index = 5;
        let is_flipped = !(0.0 <= rotation && rotation <= PI);
        let is_moving = self.player.is_moving();
        let initial_head_position = self.player.initial_positions[head_index];
        let model_group = &mut self.player.model_group;
        model_group.set_flipped(is_flipped);
        // Move state
        let head = model_group.get_model(head_index);
        let mul = ((is_flipped as i32) * 2 - 1) as f32;
        if is_moving {
            let mut head_position = head.get_position().clone();
            if !is_flipped {
                head_position.x = initial_head_position.x - 0.01;
            }
            head_position.y = initial_head_position.y - 0.01;
            head.set_position(head_position);
            head.set_rotation(mul * PI / 2.0);
        } else {
            head.set_position(initial_head_position);
            head.set_rotation(0.0);
        }
        model_group.set_rotation(rotation);
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

        camera_position
            .add(&direction.times(move_speed * f32::sqrt(length) * *delta_time as f32 / 10.0));
        self.focal_offset = player_position.minus(&camera_position);
        self.camera.set_position(camera_position);
    }
}
