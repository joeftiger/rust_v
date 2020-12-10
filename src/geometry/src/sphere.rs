use ultraviolet::Vec3;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{Container, Geometry, GeometryInfo};
use util::math::solve_quadratic;
use util::MinMaxExt;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        debug_assert!(radius > 0.0);

        Self { center, radius }
    }
}

impl Container for Sphere {
    fn contains(&self, obj: Vec3) -> bool {
        (obj - self.center).mag() < self.radius
    }
}

impl Geometry for Sphere {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        Aabb::new(self.center - offset, self.center + offset)
    }

    fn sample_surface(&self, sample: &Vec3) -> Vec3 {
        let theta = sample.x * 2.0;
        let phi = sample.y * 2.0;

        let phi_sin = phi.sin();

        let v = Vec3::new(
            self.radius * phi_sin * theta.cos(),
            self.radius * phi_sin * theta.sin(),
            self.radius * phi.cos(),
        );

        self.center + v
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let (t0, t1) = solve_quadratic(a, b, c)?;

        let t_min = f32::mmin_op2(ray.is_in_range_op(t0), ray.is_in_range_op(t1))?;

        let point = ray.at(t_min);
        let mut normal = (point - self.center).normalized();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(GeometryInfo::new(*ray, t_min, point, normal))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        if let Some((t0, t1)) = solve_quadratic(a, b, c) {
            ray.is_in_range(t0) || ray.is_in_range(t1)
        } else {
            false
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Vec3::zero(), 1.0)
    }
}
