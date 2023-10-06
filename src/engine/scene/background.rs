use crate::{texture_handler::TextureHandler, vec3::Vec3, model::Model};

pub struct Background {
    pub layers: Vec<Model>
} 

impl Background {
    pub fn construct(texture_handler: &mut TextureHandler) -> Self {
        let vertices: &[f32] = &[
            -1.0, -1.0, 1.0,
            1.0, -1.0, 1.0,
            1.0, 1.0, 1.0,
            -1.0, 1.0, 1.0
        ];
        let indices: &[u32] = &[
            0, 1, 2,
            0, 2, 3
        ];
        let mut layer1 = Model::new(vertices, indices, &[]);
        layer1.set_color(Vec3::new(0.2353, 0.6314, 0.7490)); // #3CA1BF
        let layers = vec![
            layer1
        ];
        Background {
            layers
        }
    }
    
}
