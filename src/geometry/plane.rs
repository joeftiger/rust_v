use serde::{Deserialize, Serialize};
use ultraviolet::Vec3;

use crate::geometry::{Aabb, AngularExt, Boxable, Intersectable, Intersection};
use crate::geometry::ray::Ray;
use crate::Float;

/// A geometrical plane.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Plane {
    /// The distance of the plane into the normal direction.
    pub d: Float,
    /// The normal of a plane.
    pub normal: Vec3,
}

impl Plane {
    pub fn new(d: Float, normal: Vec3) -> Self {
        Self {
            d,
            normal: normal.normalized(),
        }
    }

    /// Calculates the projected position of the given point to the plane.
    #[inline]
    #[must_use]
    pub fn project(&self, point: Vec3) -> Vec3 {
        point - (self.normal * self.distance(point) as f32)
    }

    /// Calculates the distance of the given point to the plane.
    #[inline]
    #[must_use]
    pub fn distance(&self, point: Vec3) -> Float {
        let v = point - (self.normal * self.d as f32);
        let angle = self.normal.angle_to(v);

        angle.cos() * v.mag() as Float
    }
}

impl From<(Vec3, Vec3)> for Plane {
    fn from(pos_normal: (Vec3, Vec3)) -> Self {
        let position = pos_normal.0;
        let normal = pos_normal.1.normalized();

        let angle = position.angle_to(normal);
        let d = angle.cos() * position.mag() as Float;

        Self { d, normal }
    }
}

impl Boxable for Plane {
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}

impl Intersectable<Ray> for Plane {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction) as Float;
        if denom.abs() <= Float::EPSILON {
            return None;
        }

        let t = -(self.normal.dot(ray.origin) as Float + self.d) / denom;
        if t <= Float::EPSILON {
            return None;
        }

        let position = ray.at(t);

        let mut normal = self.normal;
        if denom < 0.0 {
            normal = -normal;
        }

        Some(Intersection::new(position, normal, t))
    }
}
