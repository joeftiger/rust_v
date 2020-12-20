use crate::mc::{uniform_cone_pdf, uniform_sample_cone_frame, uniform_sample_sphere};
use crate::render::objects::emitter::{Sampleable, SurfaceSample};
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::{CoordinateSystem, Intersectable};
use std::f32::consts::PI;
use ultraviolet::{Vec2, Vec3};
use util::floats;

impl Sampleable for Sphere {
    fn surface_area(&self) -> f32 {
        4.0 * PI * self.radius * self.radius
    }

    fn sample_surface(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample {
        let dist_sq = (*point - self.center).mag_sq();
        let r2 = self.radius * self.radius;

        if dist_sq - r2 < floats::BIG_EPSILON {
            // inside the sphere
            let point = self.center + uniform_sample_sphere(sample) * self.radius;
            let normal = point.normalized();

            SurfaceSample::new(point, normal)
        } else {
            let z = -point.normalized();
            let frame = CoordinateSystem::from(&z);

            let cos_theta_max = f32::max(0.0, 1.0 - r2 / dist_sq);
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
                }
            }
        }
    }

    fn pdf(&self, ray: &Ray) -> f32 {
        let dist_sq = (ray.origin - self.center).mag_sq();
        let r2 = self.radius * self.radius;

        if dist_sq - r2 < floats::BIG_EPSILON {
            1.0 / self.surface_area()
        } else {
            let cos_theta = f32::max(0.0, 1.0 - r2 / dist_sq);

            uniform_cone_pdf(cos_theta)
        }
    }
}
