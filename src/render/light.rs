use ultraviolet::Vec3;

use crate::color::Srgb;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, Hit, GeometryInfo};
use std::ops::Mul;

pub struct Light {
    pub point: Vec3,
    shape: Box<dyn Geometry>,
    pub color: Srgb,
    pub intensity: f32,
}

impl Light {
    pub fn new(point: Vec3, shape: Box<dyn Geometry>, color: Srgb, intensity: f32) -> Self {
        Self { point, shape, color, intensity }
    }

    pub fn ray_to(&self, point: Vec3) -> Ray {
        Ray::new_simple(self.point, point - self.point)
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        (self.point - point).mag()
    }

    pub fn intensity_at(&self, point: Vec3) -> f32 {
        let distance = self.distance(point);

        self.intensity / (distance * distance)
    }

    pub fn color_at(&self, point: Vec3) -> Srgb {
        let intensity = self.intensity_at(point);

        self.color.mul(intensity)
    }
}

impl Geometry for Light {
    fn bounding_box(&self) -> Aabb {
        self.shape.bounding_box()
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.shape.intersect(ray)
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        self.shape.get_info(hit)
    }
}
