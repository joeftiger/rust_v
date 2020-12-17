use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::{IntersectionInfo, Boundable, Intersectable};
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

impl Boundable for Point {
    fn bounds(&self) -> Aabb {
        Aabb::new(self.position, self.position)
    }
}

impl Intersectable for Point {
    #[inline(always)]
    fn intersect(&self, _: &Ray) -> Option<IntersectionInfo> {
        None
    }

    #[inline(always)]
    fn intersects(&self, _ray: &Ray) -> bool {
        false
    }
}