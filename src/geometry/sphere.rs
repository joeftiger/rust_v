use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo, Hit};
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

impl Container for Sphere {
    fn contains(&self, obj: Vec3) -> bool {
        (obj - self.center).mag() < self.radius
    }
}

impl Geometry for Sphere {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        Aabb::new(self.center - offset, self.center + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let solutions = solve_quadratic(a, b, c);
        let t = solutions
            .into_iter()
            .filter(|sol| *sol > 0.0)
            .min_by(|s1, s2| s1.partial_cmp(s2).unwrap());

        t
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        let point = hit.ray.at(hit.t);
        let normal = (point - self.center).normalized();

        GeometryInfo::new(hit.ray, hit.t, point, normal)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
