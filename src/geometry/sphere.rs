use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo, Hit};
use crate::math::solve_quadratic;

#[derive(Debug, PartialEq)]
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

        let (t0, t1) = solve_quadratic(a, b, c)?;
        let t_min = t0.min(t1);

        if t_min < 0.0 || ray.t < t_min {
            None
        } else {
            Some(t_min)
        }
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        let point = hit.ray.at(hit.t);
        let normal = (point - self.center).normalized();

        GeometryInfo::new(hit, point, normal)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
