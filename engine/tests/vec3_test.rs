#[cfg(test)]
mod vec3_test {
    use lumina_engine::math::vec2::Vec2;
    use lumina_engine::math::vec3::Vec3;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let v = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_from_vec2() {
        let v2 = Vec2::new(1.0, 2.0);
        let v3 = Vec3::from_vec2(v2, 3.0);
        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn test_from_ref() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::from(&a);
        assert_eq!(a, b);
    }

    #[test]
    fn test_from_tuple() {
        let v: Vec3 = (1.0, 2.0, 3.0).into();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(2.0, 3.0, 6.0);
        assert_eq!(v.length(), 7.0);
    }

    #[test]
    fn test_length_zero() {
        assert_eq!(Vec3::zero().length(), 0.0);
    }

    #[test]
    fn test_xy() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v.xy();
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 2.0);
    }

    #[test]
    fn test_normalized() {
        let v = Vec3::new(0.0, 0.0, 5.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < EPSILON);
        assert!((n.z - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_normalized_zero_vector() {
        let v = Vec3::zero();
        let n = v.normalized();
        assert_eq!(n, Vec3::zero());
    }

    #[test]
    fn test_normalized_preserves_direction() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalized();
        assert!((n.x - 0.6).abs() < EPSILON);
        assert!((n.y - 0.8).abs() < EPSILON);
        assert!(n.z.abs() < EPSILON);
    }

    #[test]
    fn test_distance() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(a.distance(b), 5.0);
    }

    #[test]
    fn test_distance_same_point() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.distance(a), 0.0);
    }

    #[test]
    fn test_distance_3d() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 6.0, 3.0);
        assert_eq!(a.distance(b), 5.0);
    }

    #[test]
    fn test_add_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_assign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a += Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub_vec3() {
        let a = Vec3::new(5.0, 7.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a - b, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_sub_assign_vec3() {
        let mut a = Vec3::new(5.0, 7.0, 9.0);
        a -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_add_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v + 10.0, Vec3::new(11.0, 12.0, 13.0));
    }

    #[test]
    fn test_add_assign_scalar() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += 10.0;
        assert_eq!(v, Vec3::new(11.0, 12.0, 13.0));
    }

    #[test]
    fn test_sub_scalar() {
        let v = Vec3::new(11.0, 12.0, 13.0);
        assert_eq!(v - 10.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_sub_assign_scalar() {
        let mut v = Vec3::new(11.0, 12.0, 13.0);
        v -= 10.0;
        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_default() {
        let v: Vec3 = Default::default();
        assert_eq!(v, Vec3::zero());
    }

    #[test]
    fn test_add_zero() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v + Vec3::zero(), v);
    }

    #[test]
    fn test_sub_self_gives_zero() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v - v, Vec3::zero());
    }

    #[test]
    fn test_mul_by_zero() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 0.0, Vec3::zero());
    }

    #[test]
    fn test_mul_by_one() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 1.0, v);
    }

    #[test]
    fn test_neg_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(-(-v), v);
    }

    #[test]
    fn test_distance_symmetry() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a.distance(b), b.distance(a));
    }
}
