use crate::geometry::aabb::Aabb;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use ultraviolet::Vec3;

pub struct Point {
    position: Vec3,
}

impl Geometry<Ray, Intersection> for Point {
    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.position, self.position)
    }

    fn intersect(&self, _: &Ray) -> Option<Intersection> {
        None
    }
}
