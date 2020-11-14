use ultraviolet::Vec3;

use crate::geometry::aabb::Aabb;
use crate::geometry::plane::Plane2;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, DistanceExt, Geometry, GeometryInfo};
use crate::math::solve_quadratic;
use crate::util::MinMaxExt;

/// A geometrical cylinder.
#[derive(Debug, PartialEq)]
pub struct Cylinder {
    pub center: Vec3,
    pub v_caps: (Vec3, Vec3),
    pub p_caps: Plane2,
    pub radius: f32,
}

impl Cylinder {
    pub fn new2(bot: Vec3, top: Vec3, radius: f32) -> Self {
        let center = (bot + top) / 2.0;
        let axis = top - bot;

        Self::new(center, axis.normalized(), radius, axis.mag())
    }

    pub fn new(center: Vec3, axis: Vec3, radius: f32, height: f32) -> Self {
        let a = axis * (height / 2.0);
        let v_caps = (center - a, center + a);

        let cos = center.dot(axis);
        let angle = cos.acos() - std::f32::consts::FRAC_PI_2;

        let d0 = angle.sin() * center.mag();
        let d1 = d0 + height;
        let p_caps = Plane2::new(axis, d0, d1);

        Self {
            center,
            v_caps,
            p_caps,
            radius,
        }
    }

    fn check_cylinder_height(&self, ray: &Ray, t: f32) -> Option<f32> {
        ray.check_range(t)?;

        let z = self.axis().dot(ray.at(t) - self.center);

        if 2.0 * z.abs() <= self.height() {
            Some(t)
        } else {
            None
        }
    }

    fn check_cylinder(&self, ray: &Ray, t0: f32, t1: f32) -> Option<f32> {
        let a = self.check_cylinder_height(ray, t0);
        let b = self.check_cylinder_height(ray, t1);
        f32::mmin_op2(a, b)
    }

    fn check_caps(&self, ray: &Ray) -> Option<GeometryInfo> {
        let caps = self.p_caps.intersect(ray)?;
        let (c0, c1) = self.v_caps;

        if caps.point.distance(&c0) <= self.radius || caps.point.distance(&c1) <= self.radius {
            Some(caps)
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    pub fn axis(&self) -> Vec3 {
        let (c0, c1) = self.v_caps;
        (c1 - c0).normalized()
    }

    #[inline]
    #[must_use]
    pub fn height(&self) -> f32 {
        let (c0, c1) = self.v_caps;
        (c1 - c0).mag()
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::unit_z(), 1.0, 2.0)
    }
}

impl Container for Cylinder {
    fn contains(&self, obj: Vec3) -> bool {
        let z = 2.0 * self.axis().dot(obj - self.center).abs();
        self.p_caps.contains(obj) && z < self.height()
    }
}

impl Geometry for Cylinder {
    fn bounding_box(&self) -> Aabb {
        let offset = Vec3::one() * self.radius;

        let min = self.center - self.axis() * self.height() / 2.0;
        let max = self.center + self.axis() * self.height() / 2.0;
        let min_original = min;
        let min = min.min_by_component(max);
        let max = max.max_by_component(min_original);

        Aabb::new(min - offset, max + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        // Intersect with the infinite cylinder
        let dir = ray.direction;
        let oc = ray.origin - self.center;

        let dir_parallel = self.axis().dot(dir);
        let oc_parallel = self.axis().dot(oc);

        let a = dir.dot(dir) - dir_parallel * dir_parallel;
        let b = 2.0 * (dir.dot(oc) - dir_parallel * oc_parallel);
        let c = oc.dot(oc) - oc_parallel * oc_parallel - self.radius * self.radius;

        // Find the closest valid solution in front of the viewer).
        let (t0, t1) = solve_quadratic(a, b, c)?;

        // Check if the intersection height is between the caps
        let t_min = self.check_cylinder(ray, t0, t1)?;
        // Intersect with each cap (already checked against ray)
        let caps = self.check_caps(ray);

        // check against ray
        if t_min < ray.t_start || ray.t_end < t_min {
            return caps;
        }

        // check against caps
        if let Some(i) = caps {
            if i.t < t_min {
                return caps;
            }
        }

        let point = ray.at(t_min);
        let mut normal = (point - self.center) / self.radius;
        normal -= normal.dot(self.axis()) * self.axis();

        // Choose the normal's orientation to be opposite the ray's
        // (in case the ray intersects the inside surface)
        if normal.dot(ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(GeometryInfo::new(*ray, t_min, point, normal))
    }
}
