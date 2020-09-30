use ultraviolet::Vec3;

use crate::geometry::{Aabb, AngularExt, Boxable, Intersectable, Intersection, Ray};

pub struct Plane {
    pub d: f32,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(d: f32, normal: Vec3) -> Self {
        Self {
            d,
            normal: normal.normalized(),
        }
    }

    pub fn from(position: Vec3, normal: Vec3) -> Self {
        let angle = position.angle_to(normal);
        let d = angle.cos() * position.mag();

        Self {
            d,
            normal: normal.normalized(),
        }
    }
}

impl Intersectable<&Ray> for Plane {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() <= f32::EPSILON {
            return None;
        }

        let t = -(self.normal.dot(ray.origin) + self.d) / denom;
        if t <= f32::EPSILON {
            return None;
        }

        let position = ray.at(t);

        let mut normal = self.normal;
        if denom < 0.0 {
            normal = -normal;
        }

        Some(Intersection::new(position, normal))
    }
}

impl Boxable for Plane {
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
