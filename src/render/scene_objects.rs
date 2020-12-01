use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{DefaultGeometry, Geometry, GeometryInfo};
use crate::render::material::Material;

pub struct SceneObject {
    shape: Box<dyn Geometry>,
    pub material: Material,
}

impl Default for SceneObject {
    fn default() -> Self {
        Self::new(Box::new(DefaultGeometry), Material::default())
    }
}

impl SceneObject {
    pub fn new(shape: Box<dyn Geometry>, material: Material) -> Self {
        Self {
            shape,
            material
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
