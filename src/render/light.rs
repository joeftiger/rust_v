use ultraviolet::Vec3;

use crate::geometry::ray::Ray;
use crate::Spectrum;

pub struct Light {
    pub point: Vec3,
    pub color: Spectrum,
    pub intensity: f32,
}

impl Light {
    pub fn new(point: Vec3, color: Spectrum, intensity: f32) -> Self {
        Self {
            point,
            color,
            intensity,
        }
    }

    pub fn direction_from(&self, point: &Vec3) -> Vec3 {
        (self.point - *point).normalized()
    }

    pub fn direction_to(&self, point: &Vec3) -> Vec3 {
        (*point - self.point).normalized()
    }

    pub fn ray_to(&self, point: &Vec3) -> Ray {
        let diff = *point - self.point;
        Ray::new(self.point, diff.normalized(), diff.mag())
    }

    pub fn ray_from(&self, point: &Vec3) -> Ray {
        let diff = self.point - *point;
        Ray::new(self.point, diff.normalized(), diff.mag())
    }

    pub fn distance(&self, point: &Vec3) -> f32 {
        (self.point - *point).mag()
    }

    pub fn intensity_at(&self, point: &Vec3) -> Spectrum {
        let dist = self.distance(point);

        self.color * (self.intensity / (dist * dist))
    }
}
