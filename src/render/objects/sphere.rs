use crate::mc::{uniform_cone_pdf, uniform_sample_sphere, uniform_sample_cone};
use crate::render::objects::emitter::{Sampleable, SurfaceSample};
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::Intersectable;
use std::f32::consts::PI;
use ultraviolet::{Vec2, Vec3};
use util::floats;
use crate::bxdf::world_to_bxdf;

impl Sampleable for Sphere {
    fn surface_area(&self) -> f32 {
        4.0 * PI * self.radius * self.radius
    }

    fn sample_surface(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample {
        let to_center = self.center - *point;
        let dist_sq = to_center.mag_sq();
        let r2 = self.radius * self.radius;

        if dist_sq - r2 < floats::BIG_EPSILON {
            // inside the sphere
            let p = uniform_sample_sphere(sample) * self.radius;
            let normal = p.normalized();

            SurfaceSample::new(self.center + p, normal)
        } else {
            let cos_theta_max = f32::max(0.0, 1.0 - r2 / dist_sq).sqrt() / 2.0;

            let axis = to_center;

            let rotation = world_to_bxdf(&axis);
            let direction = rotation.reversed() * uniform_sample_cone(sample, cos_theta_max).normalized();
            // let direction = axis.normalized();

            let ray = Ray::new(*point, direction);

            match self.intersect(&ray) {
                Some(i) => {
                    SurfaceSample::new(i.point, i.normal)
                },
                None => {
                    // if we miss, approximate the hit of the edge
                    let t = ray.direction.dot(-*point);
                    let p = ray.at(t);
                    let normal = p.normalized();

                    SurfaceSample::new(p, normal)
                }
            }
        }
    }

    fn pdf(&self, ray: &Ray) -> f32 {
        let dist_sq = (ray.origin - self.center).mag_sq();
        let r2 = self.radius * self.radius;

        if dist_sq - r2 < floats::BIG_EPSILON {
            // inside the sphere (may happen)
            1.0 / self.surface_area()
        } else {
            let cos_theta = f32::max(0.0, 1.0 - r2 / dist_sq).sqrt();

            uniform_cone_pdf(cos_theta)
        }
    }
}
