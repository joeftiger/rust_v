use crate::render::transform::Transform;
use geometry::aabb::Aabb;

pub trait Shape {
    fn new(object_to_world: &Transform, world_to_object: &Transform) -> Self;

    fn object_bounds(&self) -> Aabb;
    fn world_bounds(&self) -> Aabb;
}