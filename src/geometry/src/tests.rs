#[cfg(test)]
#[allow(dead_code)]
mod util {
    use crate::ray::Ray;
    use ultraviolet::Vec3;

    pub fn unit_vec3s() -> [Vec3; 6] {
        [
            Vec3::unit_x(),
            -Vec3::unit_x(),
            Vec3::unit_y(),
            -Vec3::unit_y(),
            Vec3::unit_z(),
            -Vec3::unit_z(),
        ]
    }

    pub fn unit_rays(dist_from_zero: f32) -> Vec<Ray> {
        unit_vec3s()
            .iter()
            .map(|v| Ray::new(*v * dist_from_zero, -*v))
            .collect()
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod aabb {
    use crate::floats;
    use crate::aabb::*;
    use crate::ray::Ray;
    use crate::{Container, Geometry};
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
    fn default() {
        let aabb = Aabb::default();

        assert_eq!(-Vec3::one(), aabb.min);
        assert_eq!(Vec3::one(), aabb.max);
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

        assert!(floats::approx_equal(
            f32::sqrt(3.0) / 2.0,
            aabb.max_radius()
        ));
    }

    #[test]
    fn min_radius() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        assert!(floats::approx_equal(0.5, aabb.min_radius()));
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
    fn overlaps() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let min = Vec3::one() / 2.0;
        let max = Vec3::one();
        let other = Aabb::new(min, max);

        assert!(aabb.overlaps(&other));
    }

    #[test]
    fn overlaps_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let min = -Vec3::one();
        let max = Vec3::zero();
        let other = Aabb::new(min, max);

        assert!(!aabb.overlaps(&other));
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

        let bb = aabb.bounds();

        assert_eq!(min, bb.min);
        assert_eq!(max, bb.max);
    }

    #[test]
    fn intersect_side() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::one() / 2.0 + Vec3::unit_x() * 1.5, -Vec3::unit_x());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_some());
        let i = intersection.unwrap();
        assert!(floats::approx_equal(1.0, i.t));
    }

    #[test]
    fn intersect_corner() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::one() * 2.0, -Vec3::one().normalized());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_some());
        let i = intersection.unwrap();
        assert!(floats::approx_equal(f32::sqrt(3.0), i.t));
        // floating point errors if not normalized
        assert_eq!(Vec3::one().normalized(), i.point.normalized());

        // can't really test intersection.normal, since it is a bit unpredictable in corner cases!
    }

    #[test]
    fn intersect_side_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::one() / 2.0 + Vec3::unit_x() * 1.5, -Vec3::unit_y());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::unit_x() * 1.5, -Vec3::unit_x());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge_not_2() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::unit_y() * 2.0, -Vec3::one().normalized());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_from_inside_not() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::zero(), -Vec3::unit_x());
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_not_in_range() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::with(Vec3::zero(), -Vec3::unit_x(), 0.0, 0.5);
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }
}

#[cfg(test)]
mod point {
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::Geometry;
    use ultraviolet::Vec3;

    #[test]
    fn new() {
        let position = Vec3::zero();
        let point = Point::new(position);

        assert_eq!(position, point.position);
    }

    #[test]
    fn default() {
        let point = Point::default();

        assert_eq!(Vec3::zero(), point.position);
    }

    #[test]
    fn bounding_box() {
        let point = Point::default();
        let bbox = point.bounds();

        assert!(bbox.is_valid());
        assert_eq!(Vec3::zero(), bbox.min);
        assert_eq!(Vec3::zero(), bbox.max);
    }

    #[test]
    fn intersect() {
        let point = Point::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x());

        let intersection = point.intersect(&ray);

        assert!(intersection.is_none())
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod sphere {
    use crate::floats;
    use crate::aabb::Aabb;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::{Container, Geometry};
    use ultraviolet::Vec3;

    #[test]
    fn new() {
        let center = Vec3::zero();
        let radius = 1.0;
        let sphere = Sphere::new(center, radius);

        assert_eq!(center, sphere.center);
        assert_eq!(radius, sphere.radius);
    }

    #[test]
    #[should_panic]
    fn new_fail() {
        let center = Vec3::zero();
        let radius = -1.0;
        let _sphere = Sphere::new(center, radius);
    }

    #[test]
    fn default() {
        let sphere = Sphere::default();

        assert_eq!(Vec3::zero(), sphere.center);
        assert_eq!(1.0, sphere.radius);
    }

    #[test]
    fn contains() {
        let sphere = Sphere::default();
        let point = Vec3::zero();

        assert!(sphere.contains(point));
    }

    #[test]
    fn contains_not() {
        let sphere = Sphere::default();
        let point = Vec3::one();

        assert!(!sphere.contains(point));
    }

    #[test]
    fn contains_not_edge() {
        let sphere = Sphere::default();
        let point = Vec3::unit_x();

        assert!(!sphere.contains(point));
    }

    #[test]
    fn bounding_box() {
        let sphere = Sphere::default();

        assert_eq!(Aabb::default(), sphere.bounds());
    }

    #[test]
    fn intersect() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() * 2.0, -Vec3::unit_x());

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
        let i = intersection.unwrap();
        assert!(floats::approx_equal(1.0, i.t));
    }

    #[test]
    fn intersect_not() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() * 2.0, -Vec3::unit_y());

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() + Vec3::unit_y(), -Vec3::unit_x());

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
        let i = intersection.unwrap();
        assert_eq!(Vec3::unit_y(), i.point);
        assert_eq!(Vec3::unit_y(), i.normal);
    }

    #[test]
    fn intersect_inner() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x());

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
    }

    #[test]
    fn intersect_not_in_range() {
        let sphere = Sphere::default();
        let ray = Ray::with(Vec3::zero(), Vec3::unit_x(), 0.0, 0.5);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }
}
