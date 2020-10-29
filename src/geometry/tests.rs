#[cfg(test)]
mod aabb {
    use crate::floats;
    use crate::geometry::aabb::*;
    use crate::geometry::ray::Ray;
    use crate::geometry::{Container, Geometry};
    use ultraviolet::Vec3;

    #[test]
    fn new() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert_eq!(min, aabb.min);
        assert_eq!(max, aabb.max);
    }

    #[test]
    #[should_panic]
    fn new_fail() {
        let min = Vec3::one();
        let max = Vec3::zero();
        let _aabb = Aabb::new(min, max);
    }

    #[test]
    fn is_valid() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(aabb.is_valid());
    }

    #[test]
    fn is_not_valid() {
        let aabb = Aabb::inverted_infinite();

        assert!(!aabb.is_valid());
    }

    #[test]
    fn size() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert_eq!(Vec3::one(), aabb.size());
    }

    #[test]
    fn center() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert_eq!(Vec3::one() / 2.0, aabb.center());
    }

    #[test]
    fn center_big() {
        let min = Vec3::one() * f32::MIN;
        let max = Vec3::one() * f32::MAX;
        let aabb = Aabb::new(min, max);

        assert_eq!(Vec3::zero(), aabb.center());
    }

    #[test]
    fn max_radius() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(floats::approx_equal(0.8660254, aabb.max_radius()));
    }

    #[test]
    fn inner_join() {
        let min = Vec3::zero();
        let max = Vec3::one() * 2.0;
        let aabb0 = Aabb::new(min, max);

        let min = -Vec3::one();
        let max = Vec3::one();
        let aabb1 = Aabb::new(min, max);

        let inner_join = aabb0.inner_join(&aabb1);

        assert_eq!(Vec3::zero(), inner_join.min);
        assert_eq!(Vec3::one(), inner_join.max);
    }

    #[test]
    fn outer_join() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb0 = Aabb::new(min, max);

        let min = -Vec3::one();
        let max = Vec3::zero();
        let aabb1 = Aabb::new(min, max);

        let outer_join = aabb0.outer_join(&aabb1);

        assert_eq!(-Vec3::one(), outer_join.min);
        assert_eq!(Vec3::one(), outer_join.max);
    }

    #[test]
    fn contains() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(aabb.contains(Vec3::one() / 2.0));
    }

    #[test]
    fn contains_edge() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(aabb.contains(min));
        assert!(aabb.contains(max));
    }

    #[test]
    fn contains_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(!aabb.contains(Vec3::one() * 2.0));
    }

    #[test]
    fn bounding_box() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let bb = aabb.bounding_box();

        assert_eq!(min, bb.min);
        assert_eq!(max, bb.max);
    }

    #[test]
    fn intersect_side() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(
            Vec3::one() / 2.0 + Vec3::unit_x() * 1.5,
            -Vec3::unit_x(),
            f32::INFINITY,
        );
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_some());
        assert!(floats::approx_equal(1.0, intersection.unwrap()));
    }

    #[test]
    fn intersect_corner() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::one() * 2.0, -Vec3::one().normalized(), f32::INFINITY);
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_some());
        assert!(floats::approx_equal(f32::sqrt(3.0), intersection.unwrap()));
    }

    #[test]
    fn intersect_side_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(
            Vec3::one() / 2.0 + Vec3::unit_x() * 1.5,
            -Vec3::unit_y(),
            f32::INFINITY,
        );
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::unit_x() * 1.5, -Vec3::unit_x(), f32::INFINITY);
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge_not_2() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(
            Vec3::unit_y() * 2.0,
            -Vec3::one().normalized(),
            f32::INFINITY,
        );
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_from_inside_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(
            Vec3::zero(),
            -Vec3::unit_x(),
            f32::INFINITY,
        );
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }
}
