mod sphere;

use geometry::ray::Ray;
use geometry::Geometry;
use ultraviolet::{Vec2, Vec3};

pub struct SurfaceSample {
    pub point: Vec3,
    pub normal: Vec3,
}

impl SurfaceSample {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

/// A trait for objects (`Geometry` e.g.) that can sample a point on their surface.
pub trait Sampleable {
    /// The surface area of this object
    fn surface_area(&self) -> f32;

    /// Sample this object of the solid angle from `point` to the sampled point on the surface.
    fn sample(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample;

    /// Computes the PDF that the ray intersects this object.
    fn pdf(&self, ray: &Ray) -> f32;
}

/// A helper trait combining `Geometry` and `Sampleable`.
pub trait Shape: Geometry + Sampleable {}
impl<T: ?Sized> Shape for T where T: Geometry + Sampleable {}
