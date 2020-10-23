use crate::geometry::aabb::Aabb;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use crate::color::Srgb;

pub struct SceneObject<T> {
    shape: T,
    color: Srgb,
}

impl<T> SceneObject<T> {
    pub fn new(shape: T, color: Srgb) -> Self {
        Self { shape, color }
    }

    pub fn get_color(&self) -> Srgb {
        self.color.clone()
    }
}

impl<T: Geometry<Ray, Intersection>> Geometry<Ray, Intersection> for SceneObject<T> {
    fn bounding_box(&self) -> Aabb {
        self.shape.bounding_box()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }
}
