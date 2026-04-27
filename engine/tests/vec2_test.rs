#[cfg(test)]
mod vec2_test {
    use lumina_engine::math::vec2::Vec2;
    use std::f32::consts::PI;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn test_new() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_zero() {
        let v = Vec2::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
    }

    #[test]
    fn test_unit() {
        let v = Vec2::unit();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
    }

    #[test]
    fn test_uniform() {
        let v = Vec2::uniform(5.0);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 5.0);
    }

    #[test]
    fn test_from_tuple() {
        let v: Vec2 = (1.0, 2.0).into();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_eq!(Vec2::dot(&a, &b), 0.0);

        let c = Vec2::new(3.0, 4.0);
        let d = Vec2::new(2.0, 1.0);
        assert_eq!(Vec2::dot(&c, &d), 10.0);
    }

    #[test]
    fn test_dot_product_parallel() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(2.0, 0.0);
        assert_eq!(Vec2::dot(&a, &b), 2.0);
    }

    #[test]
    fn test_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_length_zero() {
        let v = Vec2::zero();
        assert_eq!(v.length(), 0.0);
    }

    #[test]
    fn test_normalized() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < EPSILON);
        assert!((n.x - 0.6).abs() < EPSILON);
        assert!((n.y - 0.8).abs() < EPSILON);
    }

    #[test]
    fn test_normalized_zero_vector() {
        let v = Vec2::zero();
        let n = v.normalized();
        assert_eq!(n, Vec2::zero());
    }

    #[test]
    fn test_scale() {
        let mut v = Vec2::new(2.0, 3.0);
        v.scale(&Vec2::new(4.0, 5.0));
        assert_eq!(v, Vec2::new(8.0, 15.0));
    }

    #[test]
    fn test_rotate_90_degrees() {
        let mut v = Vec2::new(1.0, 0.0);
        v.rotate(PI / 2.0);
        // Uses (-angle).sin(), so this is clockwise rotation
        assert!((v.x - 0.0).abs() < EPSILON);
        assert!((v.y - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_rotate_180_degrees() {
        let mut v = Vec2::new(1.0, 0.0);
        v.rotate(PI);
        assert!((v.x - (-1.0)).abs() < EPSILON);
        assert!(v.y.abs() < EPSILON);
    }

    #[test]
    fn test_rotated_does_not_mutate() {
        let v = Vec2::new(1.0, 0.0);
        let r = v.rotated(PI / 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 0.0);
        assert!((r.x - 0.0).abs() < EPSILON);
        assert!((r.y - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_rotated_full_rotation() {
        let v = Vec2::new(1.0, 0.0);
        let r = v.rotated(2.0 * PI);
        assert!((r.x - 1.0).abs() < EPSILON);
        assert!(r.y.abs() < EPSILON);
    }

    #[test]
    fn test_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec2::new(1.0, 2.0);
        a += Vec2::new(3.0, 4.0);
        assert_eq!(a, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let a = Vec2::new(5.0, 6.0);
        let b = Vec2::new(3.0, 1.0);
        assert_eq!(a - b, Vec2::new(2.0, 5.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec2::new(5.0, 6.0);
        a -= Vec2::new(3.0, 1.0);
        assert_eq!(a, Vec2::new(2.0, 5.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        let r = v * 3.0;
        assert_eq!(r, Vec2::new(6.0, 9.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut v = Vec2::new(2.0, 3.0);
        v *= 3.0;
        assert_eq!(v, Vec2::new(6.0, 9.0));
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec2::new(6.0, 9.0);
        let r = v / 3.0;
        assert_eq!(r, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut v = Vec2::new(6.0, 9.0);
        v /= 3.0;
        assert_eq!(v, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec2::new(1.0, -2.0);
        assert_eq!(-v, Vec2::new(-1.0, 2.0));
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Vec2::new(1.0, 2.0), Vec2::new(1.0, 2.0));
        assert_ne!(Vec2::new(1.0, 2.0), Vec2::new(1.0, 3.0));
        assert_ne!(Vec2::new(1.0, 2.0), Vec2::new(2.0, 2.0));
    }

    #[test]
    fn test_mul_by_zero() {
        let v = Vec2::new(5.0, 10.0);
        assert_eq!(v * 0.0, Vec2::zero());
    }

    #[test]
    fn test_add_zero() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v + Vec2::zero(), v);
    }

    #[test]
    fn test_sub_self() {
        let v = Vec2::new(5.0, 10.0);
        assert_eq!(v - v, Vec2::zero());
    }

    #[test]
    fn test_neg_neg() {
        let v = Vec2::new(1.0, -2.0);
        assert_eq!(-(-v), v);
    }

    #[test]
    fn test_large_values() {
        let v = Vec2::new(1e10, 1e10);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_negative_values() {
        let v = Vec2::new(-3.0, -4.0);
        assert_eq!(v.length(), 5.0);
    }
}
