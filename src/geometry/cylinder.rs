use ultraviolet::Vec3;

use crate::geometry::{Boxable, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::{math, Float};

// A geometrical cylinder.
pub struct Cylinder {
    pub center: Vec3,
    pub axis: Vec3,
    pub radius: Float,
    pub height: Float,
}

impl Cylinder {
    pub fn new(center: Vec3, axis: Vec3, radius: Float, height: Float) -> Self {
        Self { center, axis, radius, height }
    }
}

impl Boxable for Cylinder {
    fn bounding_box(&self) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius as f32;

        let min = self.center - self.axis * (self.height / 2.0) as f32;
        let max = self.center + self.axis * (self.height / 2.0) as f32;
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Some(Aabb::new(min - offset, max + offset))
    }
}

impl Intersectable<Ray> for Cylinder {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let dir_parallel = self.axis.dot(dir);
        let oc_parallel = self.axis.dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - (self.radius * self.radius) as f32;

        // Find the closest valid solution
        // (in front of the viewer and within the cylinder's height).
        let mut t = Float::NEG_INFINITY;
        for sol in math::solve_quadratic(a as Float, b as Float, c as Float) {
            if sol <= 0.0 {
                continue;
            }
            let z = self.axis.dot(ray.at(sol) - self.center) as Float;
            if 2.0 * z.abs() < self.height {
                t = t.min(sol);
            }
        }
        if t == Float::NEG_INFINITY {
            return None;
        }

        let position = ray.at(t);
        let mut normal = self.axis.dot((position - self.center) / self.radius as f32) * self.axis;

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(dir) > 0.0 {
            normal *= -1.0;
        }

        return Some(Intersection::new(position, normal, t));
    }
}
