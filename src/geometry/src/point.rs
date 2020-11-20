use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{Geometry, GeometryInfo};
use ultraviolet::Vec3;

#[derive(Debug, PartialEq, Default)]
pub struct Point {
    pub position: Vec3,
}

impl Point {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

impl Geometry for Point {
    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.position, self.position)
    }

    #[inline(always)]
    fn intersect(&self, _: &Ray) -> Option<GeometryInfo> {
        None
    }

    #[inline(always)]
    fn intersects(&self, _ray: &Ray) -> bool {
        false
    }
}
