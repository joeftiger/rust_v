use crate::geometry::aabb::Aabb;

pub mod aabb;
pub mod intersection;
pub mod lens;
pub mod ray;
pub mod sphere;

pub trait Container<Tin, Tout> {
    fn contains(&self, obj: Tin) -> Tout;
}

pub trait Geometry<Tin, Tout> {
    fn bounding_box(&self) -> Aabb;

    fn intersect(&self, ray: &Tin) -> Option<Tout>;
}
