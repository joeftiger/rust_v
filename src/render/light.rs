use ultraviolet::Vec3;

use crate::floats::BIG_EPSILON;
use crate::geometry::ray::Ray;
use crate::Spectrum;

pub struct Light {
    pub point: Vec3,
    pub color: Spectrum,
}

impl Light {
    pub fn new(point: Vec3, color: Spectrum) -> Self {
        Self { point, color }
    }

    pub fn direction_from(&self, point: Vec3) -> Vec3 {
        (self.point - point).normalized()
    }

    pub fn direction_to(&self, point: Vec3) -> Vec3 {
        (point - self.point).normalized()
    }

    pub fn ray_to(&self, point: Vec3) -> Ray {
        Ray::new(
            self.point,
            self.direction_to(point),
            self.distance(point) + BIG_EPSILON,
        )
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        (self.point - point).mag()
    }
}
