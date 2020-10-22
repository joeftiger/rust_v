use ultraviolet::{f32x4, Vec3, Vec3x4};

use crate::geometry::aabb::Aabb;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry};
use crate::math::solve_quadratic;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        debug_assert!(radius > 0.0);

        Self { center, radius }
    }
}

impl Container<Vec3, bool> for Sphere {
    fn contains(&self, obj: Vec3) -> bool {
        (obj - self.center).mag() < self.radius
    }
}

impl Container<Vec3x4, f32x4> for Sphere {
    fn contains(&self, obj: Vec3x4) -> f32x4 {
        let center = Vec3x4::splat(self.center);
        let radius = f32x4::splat(self.radius);

        let inside = (obj - center).mag();
        inside.cmp_lt(radius)
    }
}

impl Geometry<Ray, Intersection> for Sphere {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        Aabb::new(self.center - offset, self.center + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let solutions = solve_quadratic(a, b, c);
        let t = solutions
            .iter()
            .filter(|sol| **sol > 0.0)
            .min_by(|s1, s2| s1.partial_cmp(s2).unwrap());

        if let Some(t) = t {
            let t = *t;
            let position = ray.at(t);
            let normal = (position - self.center).normalized();

            Some(Intersection::new(t, position, normal))
        } else {
            None
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
