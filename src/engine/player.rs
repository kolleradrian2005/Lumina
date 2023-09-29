use crate::{model::Model, vec2::Vec2, transformation, texture_handler::TextureHandler };

pub struct Player {
    pub model: Model,
    position: Vec2,
    lerp_speed: f32
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

        // TODO: add textures
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
            lerp_speed: 0.0035
        }

    }
    pub fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        transformation::create_model_matrix(&self.position, &0.0, &1.0)
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn set_position(&mut self, vec: Vec2) {
        self.position = vec;
    }

    pub fn get_lerp_speed(&self) -> f32 {
        self.lerp_speed
    }
}
