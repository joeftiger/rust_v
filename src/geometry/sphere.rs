use ultraviolet::Vec3;
use crate::geometry::{Boxable, Aabb, Intersectable, Ray, Line};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Boxable for Sphere {
    fn bounding_box(&self) -> Option<Aabb> {
        let offset = Vec3::one() * self.radius;

        Some(Aabb::new(self.center - offset, self.center + offset))
    }
}

impl Intersectable<&Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;

        let a = ray.direction.mag_sq();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}

impl Intersectable<&Line> for Sphere {
    fn intersects(&self, line: &Line) -> bool {
        let oc = line.position - self.center;

        let a = line.direction.mag_sq();
        let b = 2.0 * oc.dot(line.direction);
        let c = oc.mag_sq() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0 || -(b + discriminant.sqrt()) / (2.0 * a) > 0.0
    }
}
