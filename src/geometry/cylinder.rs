use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use crate::math::solve_quadratic;
/// A geometrical cylinder.
#[derive(Debug, PartialEq)]
pub struct Cylinder {
    pub center: Vec3,
    pub axis: Vec3,
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, axis: Vec3, radius: f32, height: f32) -> Self {
        Self {
            center,
            axis,
            radius,
            height,
        }
    }

    fn check_valid_t(&self, ray: &Ray, t: f32) -> bool {
        if t <= 0.0 {
            return false;
        }

        let z = self.axis.dot(ray.at(t) - self.center);

        if 2.0 * z.abs() < self.height {
            return true;
        }

        false
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::unit_z(), 1.0, 2.0)
    }
}

impl Geometry for Cylinder {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        let min = self.center - self.axis * self.height / 2.0;
        let max = self.center + self.axis * self.height / 2.0;
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Aabb::new(min - offset, max + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let dir_parallel = self.axis.dot(dir);
        let oc_parallel = self.axis.dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        // Find the closest valid solution
        // (in front of the viewer and within the cylinder's height).
        let (t0, t1) = solve_quadratic(a, b, c)?;

        // get valid t
        let mut t_min = None;
        if self.check_valid_t(ray, t0) {
            t_min = Some(t0)
        }
        if self.check_valid_t(ray, t1) {
            if let Some(prev) = t_min {
                t_min = Some(prev.min(t1));
            } else {
                t_min = Some(t1);
            }
        }
        let t_min = t_min?;

        if ray.t < t_min {
            None
        } else {
            Some(t_min)
        }
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        let point = hit.ray.at(hit.t);
        let mut normal = (point - self.center) / self.radius;
        normal -= normal.dot(self.axis) * self.axis;

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(hit.ray.direction) > 0.0 {
            normal = -normal;
        }

        GeometryInfo::new(hit, point, normal)
    }
}
