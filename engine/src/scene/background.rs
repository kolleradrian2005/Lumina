use crate::{
    math::{vec2::Vec2, vec3::Vec3},
    model::model::Model,
    texture::{resource_provider::ResourceProvider, texture::GradientTexture},
    transformable::Transformable,
};

pub struct Background {
    pub layers: Vec<Model>,
}

impl Background {
    pub fn construct(resource_provider: &dyn ResourceProvider) -> Self {
        let mut layer1 = resource_provider.get_model("square");
        layer1.set_position(Vec3::new(0.0, 0.0, -7.5));
        layer1.set_scale(Vec2::uniform(2.0));
        layer1.set_texture(
            GradientTexture::new((0.0, 0.29, 0.43).into(), (0.0, 0.5, 0.5).into()).into(),
        );
        let layers = vec![layer1];
        Background { layers }
    }
}
