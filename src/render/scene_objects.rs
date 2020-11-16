use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Geometry, GeometryInfo};
use crate::render::bxdf::bsdf::BSDF;

pub struct SceneObject {
    shape: Box<dyn Geometry>,
    pub bsdf: BSDF,
    pub id: usize,
}

impl SceneObject {
    pub fn new(shape: Box<dyn Geometry>, bsdf: BSDF) -> Self {
        Self {
            shape,
            bsdf,
            id: usize::MAX,
        }
    }
}

impl Geometry for SceneObject {
    fn bounding_box(&self) -> Aabb {
        self.shape.bounding_box()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInfo> {
        self.shape.intersect(ray)
    }
}
