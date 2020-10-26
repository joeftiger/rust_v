use ultraviolet::Vec3;

use crate::color::Srgb;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use std::ops::Mul;
use crate::render::scene_objects::SceneObject;

pub struct Light {
    pub point: Vec3,
    pub object: SceneObject,
    pub color: Srgb,
    pub intensity: f32,
}

impl Light {
    pub fn new(point: Vec3, object: SceneObject, color: Srgb, intensity: f32) -> Self {
        Self {
            point,
            object,
            color,
            intensity,
        }
    }

    pub fn direction_from(&self, point: Vec3) -> Vec3 {
        (self.point - point).normalized()
    }

    pub fn direction_to(&self, point: Vec3) -> Vec3 {
        (point - self.point).normalized()
    }

    pub fn ray_to(&self, point: Vec3) -> Ray {
        Ray::new_simple(self.point, self.direction_to(point))
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
        self.object.bounding_box()
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.object.intersect(ray)
    }

    fn get_info(&self, hit: Hit) -> GeometryInfo {
        self.object.get_info(hit)
    }
}
