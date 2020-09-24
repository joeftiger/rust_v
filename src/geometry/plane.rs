use ultraviolet::Vec3;

use crate::geometry::{Aabb, Angular, Intersectable, Line, Ray, Boxable};

pub struct Plane {
    d: f32,
    normal: Vec3,
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
    fn intersects(&self, ray: &Ray) -> bool {
        ray.direction.angle_to(self.normal) <= f32::EPSILON
    }
}

impl Intersectable<&Line> for Plane {
    fn intersects(&self, line: &Line) -> bool {
        line.direction.angle_to(self.normal) <= f32::EPSILON
    }
}

impl Boxable for Plane {
    fn bounding_box(&self) -> Option<Aabb> {
        None
    }
}
