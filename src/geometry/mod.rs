use crate::geometry::aabb::Aabb;

pub mod ray;
pub mod aabb;
pub mod intersection;
pub mod sphere;
pub mod lens;


pub trait Container<Tin, Tout> {
    fn contains(&self, obj: Tin) -> Tout;
}

pub trait Geometry<Tin, Tout> {
    fn bounding_box(&self) -> Aabb;

    fn intersect(&self, ray: &Tin) -> Option<Tout>;
}
