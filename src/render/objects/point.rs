use crate::render::objects::emitter::{Sampleable, SurfaceSample};
use geometry::point::Point;
use geometry::ray::Ray;
use ultraviolet::{Vec2, Vec3};

impl Sampleable for Point {
    fn surface_area(&self) -> f32 {
        0.0
    }

    fn sample_surface(&self, point: &Vec3, _: &Vec2) -> SurfaceSample {
        let normal = *point - self.position;

        SurfaceSample::new(self.position, normal.normalized())
    }

    fn pdf(&self, _: &Ray) -> f32 {
        1.0
    }
}
