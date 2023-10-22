use std::f32::consts::PI;

use crate::{model::Model, vec2::Vec2, texture_handler::TextureHandler, vec3::Vec3, model_group::ModelGroup };

pub struct Player {
    pub model_group: ModelGroup,
    initial_models: Vec<Model>, // Model palette
    dest_rotation: f32,
    move_speed: f32,
    is_moving: bool,
    pub initial_positions: Vec<Vec2>,
    pub initial_scales: Vec<f32>,
}

/*
Model group indices:

    0 - left hand
    1 - legs
    2 - torso
    3 - right hand
    4 - tank
    5 - head

*/

impl Player {
    pub fn new(texture_handler: &mut TextureHandler) -> Self {
        
        let size = 2.0;
        let model_scale = 0.15;

        let vertices: &[f32] = &[
            -size / 2.0, -size / 2.0, 1.0,
            size / 2.0, -size / 2.0, 1.0,
            size / 2.0, size / 2.0, 1.0,
            -size / 2.0, size / 2.0, 1.0,
        ];

        let indices: &[u32] = &[
            0, 1, 2,
            2, 3, 0
        ];

        let uvs: &[f32] = &[
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];

        let initial_position = Vec2::new(0.0, 0.0);
        //let mut model = Model::new(vertices, indices, uvs);
        //model.set_color(Vec3::new(0.0, 1.0, 0.0));
        //let texture_names: &[&str] = &[
        //    "player0.png",
        //    "player1.png",
        //    "player2.png",
        //    "player3.png",
        //];
        //model.load_animated_texture(texture_handler, texture_names, 500);

        let mut model_group = ModelGroup::new();
        model_group.set_scale(model_scale);
        
        let mut pattern_model = Model::new(vertices, indices, uvs);
        pattern_model.set_color(Vec3::new(0.0, 1.0, 0.0));
        
        let mut head_model = pattern_model.clone();
        let mut torso_model = pattern_model.clone();
        let mut left_hand_model = pattern_model.clone();
        let mut right_hand_model = pattern_model.clone();
        let mut legs_model = pattern_model.clone();
        let mut tank_model = pattern_model.clone();
        
        let head_textures: &[&str] = &[
            "./player/head0.png",
            //d"./player/head1.png"
        ];

        head_model.load_animated_texture(texture_handler, head_textures, 250);
        torso_model.load_single_texture(texture_handler, "./player/torso.png");
        left_hand_model.load_single_texture(texture_handler, "./player/left_hand.png");
        right_hand_model.load_single_texture(texture_handler, "./player/right_hand.png");
        legs_model.load_single_texture(texture_handler, "./player/legs0.png");
        tank_model.load_single_texture(texture_handler, "./player/tank.png");
        
        let initial_scales = vec![
            0.31640625,
            0.4375,
            0.23828125,
            0.32421875,
            0.25,
            0.23828125
        ];

        let initial_positions = vec![
            Vec2::new(0.08984375, -0.03515625).times(model_scale),
            Vec2::new(-0.03125, -0.3984375).times(model_scale),
            Vec2::new(-0.05078125, 0.09765625).times(model_scale),
            Vec2::new(-0.03515625, -0.05078125).times(model_scale),
            Vec2::new(-0.2265625, 0.125).times(model_scale),
            Vec2::new(-0.02734375, 0.54296875).times(model_scale),
        ];

        let mut initial_models = vec![
            left_hand_model,
            legs_model,
            torso_model,
            right_hand_model,
            tank_model,
            head_model
        ];

        let mut index = 0;
        for model in initial_models.iter_mut() {
            model.set_scale(initial_scales[index]);
            model.set_position(initial_positions[index]);
            model_group.add_model(model.clone());
            index += 1;
        }

        model_group.set_position(initial_position);

        Player {
            model_group,
            initial_models,
            dest_rotation: 0.0,
            move_speed: 0.00035,
            is_moving: false,
            initial_positions,
            initial_scales
        }

    }

    pub fn get_position(&self) -> &Vec2 {
        &self.model_group.get_position()
    }

    pub fn set_position(&mut self, vec: Vec2) {
        self.model_group.set_position(vec);
    }

    pub fn change_position(&mut self, offset: &Vec2) {
        self.set_moving(0.0 < offset.length());
        let norm_offset = offset.normalized();
        if norm_offset.x == 0.0 && norm_offset.y == 0.0 {
            //self.model.set_rotation(0.0);
            self.dest_rotation = 0.0;
        } else {
            //self.model.set_rotation((-norm_offset.y).atan2(norm_offset.x) + PI / 2.0);
            self.dest_rotation = (-norm_offset.y).atan2(norm_offset.x) + PI / 2.0;
        }
        self.set_position(self.model_group.get_position().plus(offset));
    }

    pub fn get_move_speed(&self) -> f32 {
        self.move_speed
    }
    pub fn get_dest_rotation(&self) -> f32 {
        self.dest_rotation
    }

    pub fn set_moving(&mut self, state: bool) {
        self.is_moving = state;
    }

    pub fn is_moving(&self) -> bool {
        self.is_moving
    }
}
