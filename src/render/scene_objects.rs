use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use crate::render::bxdf::BxDF;

pub struct SceneObject {
    shape: Box<dyn Geometry>,
    pub bxdf: Box<dyn BxDF>,
    pub id: usize,
}

impl SceneObject {
    pub fn new(shape: Box<dyn Geometry>, bxdf: Box<dyn BxDF>) -> Self {
        Self { shape, bxdf, id: usize::MAX }
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
