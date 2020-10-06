use ultraviolet::Vec3;
use crate::geometry::{Boxable, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::math;
use crate::geometry::ray::{NormalRay, Ray};

// A geometrical cylinder.
pub struct Cylinder {
    pub center: Vec3,
    pub axis: Vec3,
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, axis: Vec3, radius: f32, height: f32) -> Self {
        Self { center, axis, radius, height }
    }
}

impl Boxable for Cylinder {
    fn bounding_box(&self) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius;

        let min = self.center - self.axis * self.height / 2.0;
        let max = self.center + self.axis * self.height / 2.0;
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Some(Aabb::new(min - offset, max + offset))
    }
}

impl<T: Ray> Intersectable<T> for Cylinder {
    fn intersects(&self, ray: T) -> Option<Intersection> {
        let dir = ray.direction();
        let oc = ray.origin() - self.center;

        let dir_parallel = self.axis.dot(dir);
        let oc_parallel = self.axis.dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        // Find the closest valid solution
        // (in front of the viewer and within the cylinder's height).
        let mut t = f32::NEG_INFINITY;
        for sol in math::solve_quadratic(a, b, c) {
            if sol <= 0.0 {
                continue;
            }
            let z = self.axis.dot(ray.at(sol) - self.center);
            if 2.0 * z.abs() < self.height {
                t = t.min(sol);
            }
        }
        if t == f32::NEG_INFINITY {
            return None;
        }

        let position = ray.at(t);
        let mut normal = self.axis.dot((position - self.center) / self.radius) * self.axis;

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(dir) > 0.0 {
            normal *= -1.0;
        }

        return Some(Intersection::new(position, normal, t));
    }
}
