#[cfg(test)]
mod transformation_test {
    use lumina_engine::logic::ecs::component::transform::Transform;
    use lumina_engine::math::transformation::*;
    use lumina_engine::math::vec2::Vec2;
    use lumina_engine::math::vec3::Vec3;
    use std::f32::consts::PI;

    const EPSILON: f32 = 1e-6;

    fn approx_eq_matrix(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4], eps: f32) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if (a[i][j] - b[i][j]).abs() > eps {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_translation_identity() {
        let m = create_translation_matrix(Vec3::zero());
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_translation_values() {
        let m = create_translation_matrix(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(m[3][0], 1.0);
        assert_eq!(m[3][1], 2.0);
        assert_eq!(m[3][2], 3.0);
        assert_eq!(m[3][3], 1.0);
        // Upper-left 3x3 should be identity
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[1][1], 1.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn test_rotation_zero() {
        let m = create_rotation_matrix(0.0);
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_rotation_90_degrees() {
        let m = create_rotation_matrix(PI / 2.0);
        assert!((m[0][0] - 0.0).abs() < EPSILON); // cos(pi/2)
        assert!((m[0][1] - (-1.0)).abs() < EPSILON); // -sin(pi/2)
        assert!((m[1][0] - 1.0).abs() < EPSILON); // sin(pi/2)
        assert!((m[1][1] - 0.0).abs() < EPSILON); // cos(pi/2)
    }

    #[test]
    fn test_rotation_360_is_identity() {
        let m = create_rotation_matrix(2.0 * PI);
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_scale_identity() {
        let m = create_scale_matrix(Vec2::unit());
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_scale_values() {
        let m = create_scale_matrix(Vec2::new(2.0, 3.0));
        assert_eq!(m[0][0], 2.0);
        assert_eq!(m[1][1], 3.0);
        assert_eq!(m[2][2], 1.0);
        assert_eq!(m[3][3], 1.0);
    }

    #[test]
    fn test_scale_zero() {
        let m = create_scale_matrix(Vec2::zero());
        assert_eq!(m[0][0], 0.0);
        assert_eq!(m[1][1], 0.0);
    }

    #[test]
    fn test_view_matrix_origin() {
        let m = create_view_matrix(Vec3::zero());
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_view_matrix_negates_position() {
        let m = create_view_matrix(Vec3::new(5.0, 10.0, 15.0));
        assert_eq!(m[3][0], -5.0);
        assert_eq!(m[3][1], -10.0);
        assert_eq!(m[3][2], -15.0);
    }

    #[test]
    fn test_ortographic_projection_symmetry() {
        let m = create_ortographic_projection_matrix(1.0, -1.0, 1.0);
        // For aspect_ratio=1.0: left=-0.5, right=0.5, so tx = 0
        assert!((m[0][3] - 0.0).abs() < EPSILON); // tx should be 0 for symmetric
        assert!((m[1][3] - 0.0).abs() < EPSILON); // ty should be 0 for symmetric
    }

    #[test]
    fn test_ortographic_projection_scaling() {
        let m = create_ortographic_projection_matrix(2.0, 0.0, 10.0);
        // right - left = 2.0, so 2/(r-l) = 1.0
        assert_eq!(m[0][0], 1.0);
        // top - bottom = 1.0, so 2/(t-b) = 2.0
        assert_eq!(m[1][1], 2.0);
    }

    #[test]
    fn test_transform_matrix_identity() {
        let transform = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: false,
        };
        let m = create_transform_matrix(&transform, None);
        let identity: [[f32; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!(approx_eq_matrix(&m, &identity, EPSILON));
    }

    #[test]
    fn test_transform_matrix_with_position() {
        let transform = Transform {
            position: Vec3::new(5.0, 10.0, 0.0),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: false,
        };
        let m = create_transform_matrix(&transform, None);
        assert_eq!(m[3][0], 5.0);
        assert_eq!(m[3][1], 10.0);
    }

    #[test]
    fn test_inherited_transform_no_parent() {
        let model = Transform {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: 0.5,
            scale: Vec2::new(2.0, 3.0),
            is_flipped: false,
        };
        let result = calc_inherited_transform(&model, None);
        assert_eq!(result.position, model.position);
        assert_eq!(result.rotation, model.rotation);
        assert_eq!(result.scale, model.scale);
        assert_eq!(result.is_flipped, false);
    }

    #[test]
    fn test_inherited_transform_with_parent_position() {
        let model = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: false,
        };
        let parent = Transform {
            position: Vec3::new(10.0, 20.0, 0.0),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: false,
        };
        let result = calc_inherited_transform(&model, Some(&parent));
        assert!((result.position.x - 10.0).abs() < EPSILON);
        assert!((result.position.y - 20.0).abs() < EPSILON);
    }

    #[test]
    fn test_inherited_transform_with_parent_scale() {
        let model = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::new(2.0, 3.0),
            is_flipped: false,
        };
        let parent = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::new(4.0, 5.0),
            is_flipped: false,
        };
        let result = calc_inherited_transform(&model, Some(&parent));
        assert_eq!(result.scale, Vec2::new(8.0, 15.0));
    }

    #[test]
    fn test_inherited_transform_flip_xor() {
        let model = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: true,
        };
        let parent = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: true,
        };
        let result = calc_inherited_transform(&model, Some(&parent));
        assert_eq!(result.is_flipped, false);
    }

    #[test]
    fn test_inherited_transform_flip_propagation() {
        let model = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: false,
        };
        let parent = Transform {
            position: Vec3::zero(),
            rotation: 0.0,
            scale: Vec2::unit(),
            is_flipped: true,
        };
        let result = calc_inherited_transform(&model, Some(&parent));
        assert_eq!(result.is_flipped, true);
    }
}
