use crate::acceleration_structure::{AccelerationStructure, check_intersection};
use crate::geometry::{Intersection, Intersectable};
use crate::geometry::ray::Ray;
use crate::render::Scene;

pub struct NoAcceleration();

impl<'obj> AccelerationStructure<'obj> for NoAcceleration {
    fn accelerate(&self, ray: &Ray, scene: &'obj Scene) -> Option<Intersection> {
        let mut intersections = Vec::new();

        for object in &scene.objects {
            if let Some(i) = check_intersection(ray, object) {
                intersections.push(i);
            }
        }
        if intersections.is_empty() {
            return None;
        }


        let i = intersections
            .iter()
            .min_by(|i0, i1| i0.t.unwrap().partial_cmp(&i1.t.unwrap()).unwrap())
            .unwrap();
        let clone = Intersection::new(
            i.position.unwrap(),
            i.normal.unwrap(),
            i.t.unwrap(),
        );
        Some(clone)
    }
}
