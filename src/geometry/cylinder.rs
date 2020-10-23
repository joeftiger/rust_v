use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use crate::math::solve_quadratic;

/// A geometrical cylinder.
#[derive(Debug)]
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
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::unit_z(), 1.0, 2.0)
    }
}

impl Geometry<Ray, Intersection> for Cylinder {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        let min = self.center - self.axis * self.height / 2.0;
        let max = self.center + self.axis * self.height / 2.0;
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Aabb::new(min - offset, max + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let dir_parallel = self.axis.dot(dir);
        let oc_parallel = self.axis.dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        // Find the closest valid solution
        // (in front of the viewer and within the cylinder's height).
        let solutions = solve_quadratic(a, b, c);
        let t = solutions
            .iter()
            .filter(|sol| **sol > 0.0)
            .min_by(|s1, s2| s1.partial_cmp(s2).unwrap());

        if let Some(t) = t {
            let t = *t;
            let point = ray.at(t);
            let mut normal = self.axis.dot((point - self.center) / self.radius) * self.axis;

            // Choose the normal's orientation to be opposite the ray's
            // (in case the ray intersects the inside surface)
            if normal.dot(dir) > 0.0 {
                normal *= -1.0;
            }

            return Some(Intersection::new(t, point, normal));
        }

        None
    }
}
