use crate::{model::Model, vec2::Vec2, transformation, texture_handler::TextureHandler, vec3::Vec3 };

pub struct Player {
    pub model: Model,
    position: Vec2,
    move_speed: f32
}

impl Player {
    pub fn new(texture_handler: &mut TextureHandler) -> Self {
        let vertices: &[f32] = &[
            -0.15, -0.15, 1.0,
            0.15, -0.15, 1.0,
            0.15, 0.15, 1.0,
            -0.15, 0.15, 1.0,
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

        let mut model = Model::new(vertices, indices, uvs);

        model.set_color(Vec3::new(0.0, 1.0, 0.0));

        let texture_names: &[&str] = &[
            "player0.png",
            "player1.png",
            "player2.png",
            "player3.png",
        ];

        model.load_animated_texture(texture_handler, texture_names, 500);

        let initial_position = Vec2::new(0.0, 0.0);
        
        Player {
            model,
            position: initial_position,
            move_speed: 0.00035
        }

    }
    pub fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_model_matrix(
            &self.model.get_position().plus(&self.position),
            &0.0,
            &1.0
        )
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, vec: Vec2) {
        self.position = vec;
    }

    pub fn change_position(&mut self, offset: &Vec2) {
        self.position = self.position.plus(offset);
    }

    pub fn get_move_speed(&self) -> f32 {
        self.move_speed
    }
}
