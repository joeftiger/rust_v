use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use ultraviolet::Vec3;

#[derive(Debug, PartialEq)]
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

    fn intersect(&self, _: &Ray) -> Option<f32> {
        None
    }

    fn get_info(&self, _hit: Hit) -> GeometryInfo {
        unimplemented!()
    }
}
