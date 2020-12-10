use ultraviolet::{Rotor3, Vec3};
use util::math::solve_quadratic;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{Container, Geometry, GeometryInfo};

/// A geometrical cylinder.
#[derive(Debug, PartialEq)]
pub struct Cylinder {
    pub bot: Vec3,
    pub top: Vec3,
    pub radius: f32,
}

impl Cylinder {
    pub fn new(bot: Vec3, top: Vec3, radius: f32) -> Self {
        Self { bot, top, radius }
    }

    fn check_height(&self, ray: &Ray, t: f32) -> Option<(f32, Vec3)> {
        ray.is_in_range_op(t)?;

        let point = ray.at(t);
        if 2.0 * self.axis().dot(point - self.center()).abs() <= self.height() {
            Some((t, point))
        } else {
            None
        }
    }

    fn check_cylinder(&self, ray: &Ray, t0: f32, t1: f32) -> Option<(f32, Vec3)> {
        let closest = self.check_height(ray, t0);
        if closest.is_some() {
            closest
        } else {
            self.check_height(ray, t1)
        }
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Vec3 {
        (self.top + self.bot) / 2.0
    }

    #[inline]
    #[must_use]
    pub fn axis(&self) -> Vec3 {
        (self.top - self.bot).normalized()
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> f32 {
        (self.top - self.bot).mag()
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::unit_y(), 1.0)
    }
}

impl Container for Cylinder {
    fn contains(&self, obj: Vec3) -> bool {
        let z = self.axis().dot(obj - self.center());

        z < self.radius && 2.0 * z.abs() <= self.height()
    }
}

impl Geometry for Cylinder {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        let min = (self.bot - offset).min_by_component(self.bot + offset);
        let max = self.top + offset.max_by_component(self.top + offset);
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Aabb::new(min - offset, max + offset)
    }

    fn sample_surface(&self, sample: &Vec3) -> Vec3 {
        let rot = Rotor3::from_rotation_between(Vec3::unit_y(), self.axis());

        let theta = sample.x * 2.0;

        let v = Vec3::new(
            self.radius * theta.cos(),
            self.height() * sample.y,
            self.radius * theta.sin(),
        );

        self.bot + rot * v
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        let dir = ray.direction;
        let oc = ray.origin - self.center();

        let dir_parallel = self.axis().dot(dir);
        let oc_parallel = self.axis().dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        let (t0, t1) = solve_quadratic(a, b, c)?;

        let (t, point) = self.check_cylinder(ray, t0, t1)?;
        let mut normal = (point - self.center()) / self.radius;
        normal -= normal.dot(self.axis()) * self.axis();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        normal.normalize();

        Some(GeometryInfo::new(*ray, t, point, normal))
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let dir = ray.direction;
        let oc = ray.origin - self.center();

        let dir_parallel = self.axis().dot(dir);
        let oc_parallel = self.axis().dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        if let Some((t0, t1)) = solve_quadratic(a, b, c) {
            self.check_cylinder(ray, t0, t1).is_some()
        } else {
            false
        }
    }
}
