use crate::color::Srgb;
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};

pub struct SceneObject {
    shape: Box<dyn Geometry>,
    pub color: Srgb,
}

impl SceneObject {
    pub fn new(shape: Box<dyn Geometry>, color: Srgb) -> Self {
        Self { shape, color }
    }
}

impl Geometry for SceneObject {
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
