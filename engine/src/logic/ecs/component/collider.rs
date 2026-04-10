use crate::{logic::ecs::component::component::Component, math::vec2::Vec2};

#[derive(Clone, PartialEq)]
pub enum ColliderShape {
    Rect { width: f32, height: f32 },
    Capsule2D { width: f32, height: f32 },
}

#[derive(Component, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub offset: Vec2,
    pub boundary_points: Vec<Vec2>,
    //pub is_static: bool,
}

const TRESHOLD: f32 = 1e-6;

impl Collider {
    pub fn intersect(
        &self,
        pos_a: Vec2,
        scale_a: Vec2,
        rot_a: f32,
        other: &Collider,
        pos_b: Vec2,
        scale_b: Vec2,
        rot_b: f32,
    ) -> Option<(f32, Vec2)> {
        let (width_a, height_a) = self.scaled_dims(scale_a);
        let (width_b, height_b) = other.scaled_dims(scale_b);

        match (&self.shape, &other.shape) {
            (ColliderShape::Rect { .. }, ColliderShape::Rect { .. }) => Self::intersect_rect_rect(
                pos_a, rot_a, width_a, height_a, pos_b, rot_b, width_b, height_b,
            ),
            (ColliderShape::Rect { .. }, ColliderShape::Capsule2D { .. }) => {
                Self::intersect_rect_capsule(
                    pos_a, rot_a, width_a, height_a, pos_b, rot_b, width_b, height_b,
                )
            }
            (ColliderShape::Capsule2D { .. }, ColliderShape::Rect { .. }) => {
                Self::intersect_rect_capsule(
                    pos_b, rot_b, width_b, height_b, pos_a, rot_a, width_a, height_a,
                )
                .map(|(dist, normal)| (dist, -normal))
            }
            (ColliderShape::Capsule2D { .. }, ColliderShape::Capsule2D { .. }) => {
                Self::intersect_capsule_capsule(
                    pos_a, rot_a, width_a, height_a, pos_b, rot_b, width_b, height_b,
                )
            }
        }
    }

    fn scaled_dims(&self, scale: Vec2) -> (f32, f32) {
        match self.shape {
            ColliderShape::Rect { width, height } => (width * scale.x, height * scale.y),
            ColliderShape::Capsule2D { width, height } => (width * scale.x, height * scale.y),
        }
    }

    fn rect_corners(pos: Vec2, rot: f32, width: f32, height: f32) -> [Vec2; 4] {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        [
            Vec2::new(-half_width, half_height).rotated(rot) + pos,
            Vec2::new(half_width, half_height).rotated(rot) + pos,
            Vec2::new(half_width, -half_height).rotated(rot) + pos,
            Vec2::new(-half_width, -half_height).rotated(rot) + pos,
        ]
    }

    fn capsule_spine(pos: Vec2, rot: f32, width: f32, height: f32) -> (Vec2, Vec2) {
        let half_spine = (height - width) / 2.0;
        (
            Vec2::new(0.0, half_spine).rotated(rot) + pos,
            Vec2::new(0.0, -half_spine).rotated(rot) + pos,
        )
    }

    fn project_onto_axis(points: &[Vec2], axis: Vec2) -> (f32, f32) {
        points
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), point| {
                let projection = Vec2::dot(point, &axis);
                (min.min(projection), max.max(projection))
            })
    }

    fn closest_on_segment(point: Vec2, seg_a: Vec2, seg_b: Vec2) -> Vec2 {
        let seg_vec = seg_b - seg_a;
        let t =
            (Vec2::dot(&(point - seg_a), &seg_vec) / Vec2::dot(&seg_vec, &seg_vec)).clamp(0.0, 1.0);
        seg_a + seg_vec * t
    }

    // Find shortest push-out normal
    fn push_out_of_rect(point: Vec2, half_width: f32, half_height: f32) -> Vec2 {
        let dists = [
            (half_width - point.x, Vec2::new(1.0, 0.0)),
            (point.x + half_width, Vec2::new(-1.0, 0.0)),
            (half_height - point.y, Vec2::new(0.0, 1.0)),
            (point.y + half_height, Vec2::new(0.0, -1.0)),
        ];
        let (_, push_out_normal) = dists
            .iter()
            .min_by(|(dist_1, _), (dist_2, _)| dist_1.partial_cmp(dist_2).unwrap())
            .unwrap();
        *push_out_normal
    }

    fn closest_points_segments(
        start_a: Vec2,
        end_a: Vec2,
        start_b: Vec2,
        end_b: Vec2,
    ) -> (Vec2, Vec2) {
        let dir_a = end_a - start_a; // Direction of segment A
        let dir_b = end_b - start_b; // Direction of segment B
        let start_b_to_a = start_a - start_b; // Vector between start points of segments (gap)

        let len_sq_a = Vec2::dot(&dir_a, &dir_a); // Squared length of segment A
        let len_sq_b = Vec2::dot(&dir_b, &dir_b); // Squared length of segment B
        let dir_a_dot_b = Vec2::dot(&dir_a, &dir_b); // How parallel the directions are
        let dir_a_dot_gap = Vec2::dot(&dir_a, &start_b_to_a); // Projection of gap onto A
        let dir_b_dot_gap = Vec2::dot(&dir_b, &start_b_to_a); // Projection of gap onto B

        let (s, t) = if len_sq_a <= TRESHOLD && len_sq_b <= TRESHOLD {
            // Both segments are points
            (0.0, 0.0)
        } else if len_sq_a <= TRESHOLD {
            // A is a point, B is a segment
            (0.0, (dir_b_dot_gap / len_sq_b).clamp(0.0, 1.0))
        } else if len_sq_b <= TRESHOLD {
            // A is a segment, B is a point
            ((-dir_a_dot_gap / len_sq_a).clamp(0.0, 1.0), 0.0)
        } else {
            // v = start_b_to_a + t * dir_b - s * dir_a should be perpendicular to both segments
            // -> v . dir_a = 0 and v . dir_b = 0
            // Solve for s and t using Cramer's rule
            let denom = len_sq_a * len_sq_b - dir_a_dot_b * dir_a_dot_b;
            let mut s = if denom.abs() <= TRESHOLD {
                // Segments are parallel, choose arbitrary s
                0.0
            } else {
                ((dir_a_dot_b * dir_b_dot_gap - len_sq_b * dir_a_dot_gap) / denom).clamp(0.0, 1.0)
            };
            let mut t = (dir_a_dot_b * s + dir_b_dot_gap) / len_sq_b;
            if t < 0.0 {
                t = 0.0;
                s = (-dir_a_dot_gap / len_sq_a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((dir_a_dot_b - dir_a_dot_gap) / len_sq_a).clamp(0.0, 1.0);
            }
            (s, t)
        };

        (start_a + dir_a * s, start_b + dir_b * t)
    }

    fn intersect_rect_rect(
        pos_a: Vec2,
        rot_a: f32,
        width_a: f32,
        height_a: f32,
        pos_b: Vec2,
        rot_b: f32,
        width_b: f32,
        height_b: f32,
    ) -> Option<(f32, Vec2)> {
        let corners_a = Self::rect_corners(pos_a, rot_a, width_a, height_a);
        let corners_b = Self::rect_corners(pos_b, rot_b, width_b, height_b);
        let axes = [
            Vec2::new(rot_a.cos(), rot_a.sin()),
            Vec2::new(-rot_a.sin(), rot_a.cos()),
            Vec2::new(rot_b.cos(), rot_b.sin()),
            Vec2::new(-rot_b.sin(), rot_b.cos()),
        ];
        let mut min_overlap = f32::INFINITY;
        let mut collision_normal = Vec2::zero();

        for axis in axes {
            let (min_a, max_a) = Self::project_onto_axis(&corners_a, axis);
            let (min_b, max_b) = Self::project_onto_axis(&corners_b, axis);
            let overlap = (max_a.min(max_b)) - (min_a.max(min_b));
            if overlap <= 0.0 {
                return None; // Separating axis found, no collision
            }
            if overlap < min_overlap {
                min_overlap = overlap;
                collision_normal = axis;
            }
        }
        if Vec2::dot(&(pos_a - pos_b), &collision_normal) < 0.0 {
            collision_normal *= -1.0; // Normal should point from B to A
        }
        Some((min_overlap, collision_normal))
    }

    fn intersect_rect_capsule(
        pos_rect: Vec2,
        rot_rect: f32,
        width_rect: f32,
        height_rect: f32,
        pos_capsule: Vec2,
        rot_capsule: f32,
        width_capsule: f32,
        height_capsule: f32,
    ) -> Option<(f32, Vec2)> {
        let radius = width_capsule / 2.0;
        let half_width_rect = width_rect / 2.0;
        let half_height_rect = height_rect / 2.0;

        let (spine_1, spine_2) =
            Self::capsule_spine(pos_capsule, rot_capsule, width_capsule, height_capsule);

        let spine_1_localized = (spine_1 - pos_rect).rotated(-rot_rect);
        let spine_2_localized = (spine_2 - pos_rect).rotated(-rot_rect);

        let rect_corners = [
            Vec2::new(-half_width_rect, half_height_rect),
            Vec2::new(half_width_rect, half_height_rect),
            Vec2::new(half_width_rect, -half_height_rect),
            Vec2::new(-half_width_rect, -half_height_rect),
            Vec2::zero(),
        ];
        let candidates: Vec<Vec2> = rect_corners
            .iter()
            .map(|&corner| Self::closest_on_segment(corner, spine_1_localized, spine_2_localized))
            .chain([spine_1_localized, spine_2_localized])
            .collect();

        let (closest_spine_point, closest_rect_point, closest_points_dist) = candidates
            .iter()
            .map(|&candidate| {
                let rect_boundary_point = Vec2::new(
                    candidate.x.clamp(-half_width_rect, half_width_rect),
                    candidate.y.clamp(-half_height_rect, half_height_rect),
                );
                (
                    candidate,
                    rect_boundary_point,
                    (candidate - rect_boundary_point).length(),
                )
            })
            .min_by(|(_, _, len_1), (_, _, len_2)| len_1.partial_cmp(len_2).unwrap())
            .unwrap();

        if closest_points_dist >= radius {
            return None;
        }

        let local_normal = if closest_points_dist < TRESHOLD {
            -Self::push_out_of_rect(closest_spine_point, half_width_rect, half_height_rect)
        } else {
            (closest_rect_point - closest_spine_point).normalized()
        };
        Some((radius - closest_points_dist, local_normal.rotated(rot_rect)))
    }

    fn intersect_capsule_capsule(
        pos_a: Vec2,
        rot_a: f32,
        width_a: f32,
        height_a: f32,
        pos_b: Vec2,
        rot_b: f32,
        width_b: f32,
        height_b: f32,
    ) -> Option<(f32, Vec2)> {
        let radius_a = width_a / 2.0;
        let radius_b = width_b / 2.0;
        let radius_sum = radius_a + radius_b;

        let (spine_a_1, spine_a_2) = Self::capsule_spine(pos_a, rot_a, width_a, height_a);
        let (spine_b_1, spine_b_2) = Self::capsule_spine(pos_b, rot_b, width_b, height_b);

        let (closest_a, closest_b) =
            Self::closest_points_segments(spine_a_1, spine_a_2, spine_b_1, spine_b_2);
        let diff = closest_a - closest_b;
        let dist = diff.length();

        if dist >= radius_sum {
            return None;
        }

        let normal = if dist < TRESHOLD {
            Vec2::new(0.0, 1.0)
        } else {
            diff / dist
        };

        Some((radius_sum - dist, normal))
    }

    pub fn compute_boundary_points(&mut self, pos: Vec2, scale: Vec2, rotation: f32) -> Vec<Vec2> {
        // TODO: improve resolution
        let (width, height) = self.scaled_dims(scale);
        match self.shape {
            ColliderShape::Rect { .. } => Self::rect_corners(pos, rotation, width, height).to_vec(),
            ColliderShape::Capsule2D { .. } => {
                let radius = width / 2.0;
                let (spine_top, spine_bottom) = Self::capsule_spine(pos, rotation, width, height);
                vec![
                    spine_top + Vec2::new(-radius, 0.0).rotated(rotation),
                    spine_top + Vec2::new(0.0, radius).rotated(rotation),
                    spine_top + Vec2::new(radius, 0.0).rotated(rotation),
                    spine_bottom + Vec2::new(radius, 0.0).rotated(rotation),
                    spine_bottom + Vec2::new(0.0, -radius).rotated(rotation),
                    spine_bottom + Vec2::new(-radius, 0.0).rotated(rotation),
                ]
            }
        }
    }
}

impl From<ColliderShape> for Collider {
    fn from(shape: ColliderShape) -> Self {
        Self {
            shape,
            offset: Vec2::zero(),
            boundary_points: Vec::new(),
        }
    }
}
