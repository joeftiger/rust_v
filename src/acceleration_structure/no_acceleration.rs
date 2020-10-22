use crate::acceleration_structure::{AccelerationStructure, check_intersection};
use crate::geometry::Intersection;
use crate::geometry::ray::Ray;
use crate::render::Scene;
use crate::physics::rgb::SRGB;

pub struct NoAcceleration();

impl AccelerationStructure for NoAcceleration {
    fn accelerate(&self, ray: &Ray, scene: &Scene) -> (Option<Intersection>, Option<SRGB>) {
        let mut intersections = Vec::new();

        for object in &scene.objects {
            if let Some(i) = check_intersection(ray, object) {
                intersections.push((i, object));
            }
        }
        if intersections.is_empty() {
            return (None, None);
        }


        let i = intersections
            .iter()
            .min_by(|i0, i1| i0.0.t.unwrap().partial_cmp(&i1.0.t.unwrap()).unwrap())
            .unwrap();

        let intersection = Intersection::new(
            i.0.position.unwrap(),
            i.0.normal.unwrap(),
            i.0.t.unwrap(),
        );
        let srgb = i.1.material();

        (Some(intersection), Some(srgb))
    }
}
