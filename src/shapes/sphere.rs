use crate::shapes::{Sampleable, SurfaceSample};
use geometry::sphere::Sphere;
use ultraviolet::{Vec2, Vec3};
use geometry::ray::Ray;
use std::f32::consts::PI;
use geometry::{Intersectable, Container, CoordinateSystem};
use crate::mc::{uniform_sample_sphere, uniform_sample_cone_frame};

impl Sampleable for Sphere {
    fn surface_area(&self) -> f32 {
        4.0 * PI * self.radius * self.radius
    }

    fn sample(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample {
        if self.contains(point) {
            let point = self.center + uniform_sample_sphere(sample) * self.radius;
            let normal = point.normalized();

            SurfaceSample::new(point, normal)
        } else {
            let z = -point.normalized();
            let frame = CoordinateSystem::from(&z);

            let cos_theta_max = f32::max(0.0, 1.0 - self.radius * self.radius / point.mag_sq());
            let direction = uniform_sample_cone_frame(sample, cos_theta_max, &frame).normalized();

            let ray = Ray::new(*point, direction);

            match self.intersect(&ray) {
                Some(i) => SurfaceSample::new(i.point, i.normal),
                None => {
                    // if we miss, approximate the hit of the edge
                    let t = ray.direction.dot(-*point);
                    let point = ray.at(t);
                    let normal = point.normalized();

                    SurfaceSample::new(point, normal)
                },
            }
        }
    }

    fn pdf(&self, ray: &Ray) -> f32 {
        if let Some(i) = self.intersect(ray) {
            let dist_sq = (i.point - ray.origin).mag_sq();
            let dot = i.normal.dot(-ray.direction);
            let area = self.surface_area();

            dist_sq / (dot.abs() * area)
        } else {
            0.0
        }
    }
}