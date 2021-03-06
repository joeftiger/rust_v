use ultraviolet::Vec3;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{Container, Intersection, Boundable, Intersectable};
use util::math::solve_quadratic;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        debug_assert!(!center.x.is_nan());
        debug_assert!(!center.y.is_nan());
        debug_assert!(!center.z.is_nan());
        debug_assert!(!radius.is_nan());
        debug_assert!(radius > 0.0);

        Self { center, radius }
    }
}

impl Boundable for Sphere {
    #[inline]
    fn bounds(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        Aabb::new(self.center - offset, self.center + offset)
    }
}

impl Container for Sphere {
    fn contains(&self, obj: &Vec3) -> bool {
        (*obj - self.center).mag() < self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let (t_min, t_max) = solve_quadratic(a, b, c)?;
        let t;
        if ray.is_in_range(t_min) {
            t = t_min;
        } else if ray.is_in_range(t_max) {
            t = t_max;
        } else {
            return None;
        }

        let point = ray.at(t);
        let normal = (point - self.center).normalized();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        // if normal.dot(ray.direction) > 0.0 {
        //     normal = -normal;
        // }

        Some(Intersection::new(*ray, t, point, normal))
    }

    #[inline]
    fn intersects(&self, ray: &Ray) -> bool {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        if let Some((t0, t1)) = solve_quadratic(a, b, c) {
            ray.is_in_range(t0) || ray.is_in_range(t1)
        } else {
            false
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
