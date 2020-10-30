#[allow(clippy::float_cmp)]
#[cfg(test)]
mod aabb {
    use crate::floats;
    use crate::geometry::aabb::*;
    use crate::geometry::ray::Ray;
    use crate::geometry::{Container, Geometry, Hit};
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

        let ray = Ray::new(Vec3::zero(), -Vec3::unit_x(), f32::INFINITY);
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_not_in_range() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let ray = Ray::new(Vec3::zero(), -Vec3::unit_x(), 0.5);
        let intersection = aabb.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn get_info_side() {
        let min = Vec3::zero();
        let max = Vec3::one();
        let aabb = Aabb::new(min, max);

        let point = Vec3::one() / 2.0 + Vec3::unit_x() * 0.5;

        let ray = Ray::new(point + Vec3::unit_x(), -Vec3::unit_x(), f32::INFINITY);
        let hit = Hit::new(ray, 1.0);

        let info = aabb.get_info(hit);

        assert_eq!(hit.ray, info.ray);
        assert_eq!(hit.t, info.t);
        assert_eq!(point, info.point);
        assert_eq!(-ray.direction, info.normal);
    }
}

#[cfg(test)]
mod point {
    use crate::geometry::point::Point;
    use ultraviolet::Vec3;
    use crate::geometry::{Geometry, Hit};
    use crate::geometry::ray::Ray;

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
        let bbox = point.bounding_box();

        assert!(bbox.is_valid());
        assert_eq!(Vec3::zero(), bbox.min);
        assert_eq!(Vec3::zero(), bbox.max);
    }

    #[test]
    fn intersect() {
        let point = Point::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x(), f32::INFINITY);

        let intersection = point.intersect(&ray);

        assert!(intersection.is_none())
    }

    #[test]
    #[should_panic]
    fn get_info() {
        let point = Point::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x(), f32::INFINITY);
        let hit = Hit::new(ray, f32::default());

        let _info = point.get_info(hit);
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod sphere {
    use ultraviolet::Vec3;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::{Container, Geometry, Hit};
    use crate::geometry::aabb::Aabb;
    use crate::geometry::ray::Ray;
    use crate::floats;

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

        assert_eq!(Aabb::default(), sphere.bounding_box());
    }

    #[test]
    fn intersect() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() * 2.0, -Vec3::unit_x(), f32::INFINITY);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
        assert!(floats::approx_equal(1.0, intersection.unwrap()));
    }

    #[test]
    fn intersect_not() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() * 2.0, -Vec3::unit_y(), f32::INFINITY);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_edge() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() + Vec3::unit_y(), -Vec3::unit_x(), f32::INFINITY);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_some());
    }

    #[test]
    fn intersect_inner_not() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x(), f32::INFINITY);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn intersect_not_in_range() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::zero(), Vec3::unit_x(), 0.5);

        let intersection = sphere.intersect(&ray);

        assert!(intersection.is_none());
    }

    #[test]
    fn get_info() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() * 2.0, -Vec3::unit_x(), f32::INFINITY);

        let t = sphere.intersect(&ray).unwrap();
        let hit = Hit::new(ray, t);

        let info = sphere.get_info(hit);

        assert_eq!(ray, info.ray);
        assert_eq!(t, info.t);
        assert_eq!(Vec3::unit_x(), info.point);
        assert_eq!(Vec3::unit_x(), info.normal);
    }

    #[test]
    fn get_info_2() {
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::unit_x() + Vec3::unit_y(), -Vec3::unit_x(), f32::INFINITY);

        let t = sphere.intersect(&ray).unwrap();
        let hit = Hit::new(ray, t);

        let info = sphere.get_info(hit);

        assert_eq!(ray, info.ray);
        assert_eq!(t, info.t);
        assert_eq!(Vec3::unit_y(), info.point);
        assert_eq!(Vec3::unit_y(), info.normal);
    }
}
