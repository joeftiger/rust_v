use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo};
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

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let (t0, t1) = solve_quadratic(a, b, c)?;
        let t_min = t0.min(t1);

        if t_min < 0.0 || t_min < ray.t_start || ray.t_end < t_min {
            return None;
        }

        let point = ray.at(t_min);
        let mut normal = (point - self.center).normalized();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(GeometryInfo::new(*ray, t_min, point, normal))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
