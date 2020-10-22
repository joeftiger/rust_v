use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

use crate::geometry::{Aabb, Boxable, Intersectable, Intersection};
use crate::geometry::ray::Ray;
use crate::math::solve_quadratic;

/// A geometrical sphere.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sphere {
    /// The center of the sphere.
    pub center: Vec3,
    /// The radius of the sphere.
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}

impl Boxable for Sphere {
    fn bounding_box(&self) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius;

        Some(Aabb::new(self.center - offset, self.center + offset))
    }
}

impl Intersectable<Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
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

            Some(Intersection::new(position, normal, t))
        } else {
            None
        }
    }
}
