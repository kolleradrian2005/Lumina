use super::{math::vec2::Vec2, scene::world::component::transform_component::TransformComponent};

#[derive(Clone, Debug)]
pub struct Collider {
    pub points: [Vec2; 4],
    pub transformed_points: [Vec2; 4],
    flipped: bool,
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl Collider {
    pub fn rect(width: f32, height: f32, offset: Vec2) -> Self {
        let a = (-width / 2.0 + offset.x, height / 2.0 + offset.y).into();
        let b = (width / 2.0 + offset.x, height / 2.0 + offset.y).into();
        let c = (width / 2.0 + offset.x, -height / 2.0 + offset.y).into();
        let d = (-width / 2.0 + offset.x, -height / 2.0 + offset.y).into();

        let points = [a, b, c, d];

        Collider {
            points,
            transformed_points: points,
            flipped: false,
            position: Vec2::zero(),
            rotation: 0.0,
            scale: Vec2::uniform(1.0),
        }
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped;
        self.update_transformation();
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
        self.update_transformation();
    }

    pub fn set_rotation(&mut self, rot: f32) {
        self.rotation = rot;
        self.update_transformation();
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.update_transformation();
    }

    // Caching position
    pub fn update_transformation(&mut self) {
        for (index, point) in self.points.iter().enumerate() {
            let mut transformed_point = point.clone();
            if self.flipped {
                transformed_point.x *= -1.0;
            }
            transformed_point.scale(&self.scale);
            transformed_point.rotate(self.rotation);
            transformed_point += self.position;
            self.transformed_points[index] = transformed_point;
        }
    }

    pub fn update(&mut self, transform: TransformComponent) {
        for (index, point) in self.points.iter().enumerate() {
            let mut transformed_point = point.clone();
            if transform.is_flipped {
                transformed_point.x *= -1.0;
            }
            transformed_point.scale(&transform.scale);
            transformed_point.rotate(transform.rotation);
            transformed_point += transform.position.xy();
            self.transformed_points[index] = transformed_point;
        }
    }
}
