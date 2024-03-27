use crate::engine::{math::{vec2::Vec2, vec3::Vec3}, model::model::Model, transformable::Transformable};

pub struct UiModel {
    pub model: Model,
    pub size: (f32, f32),
    pub margin: (f32, f32)
}

impl UiModel {
    pub fn bake_model(self, aspect_ratio: f32) -> Model {
        let mut model = self.model.clone();
        model.set_scale(Vec2::new(self.size.0, self.size.1 * aspect_ratio));
        model.set_position(
            Vec3::new(
                -1.0 + self.size.0 / 2.0 + self.margin.0,
                1.0 - self.size.1 * aspect_ratio / 2.0 - self.margin.1 * aspect_ratio,
                0.0
            )
        );
        model
    }
}
