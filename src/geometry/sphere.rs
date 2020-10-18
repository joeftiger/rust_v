use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

use crate::geometry::{Aabb, Boxable, Intersectable, Intersection};
use crate::geometry::ray::Ray;
use crate::Float;

/// A geometrical sphere.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Sphere {
    /// The center of the sphere.
    pub center: Vec3,
    /// The radius of the sphere.
    pub radius: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Self {
        Sphere { center, radius }
    }
}

impl Boxable for Sphere {
    fn bounding_box(&self) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius as f32;

        Some(Aabb::new(self.center - offset, self.center + offset))
    }
}

impl Intersectable<Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;

        let a = ray.direction.mag_sq() as Float;
        let b = 2.0 * oc.dot(ray.direction) as Float;
        let c = oc.mag_sq() as Float - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let t = -b * discriminant.sqrt() / (2.0 * a);
        let position = ray.at(t);
        let normal = (position - self.center).normalized();

        Some(Intersection::new(position, normal, t))
    }
}
