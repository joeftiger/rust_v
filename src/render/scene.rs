use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use crate::store::Store;

/// Consists of
/// - info: [GeometryInfo](../geometry/struct.GeometryInfo.html)
pub struct SceneIntersection {
    pub info: GeometryInfo,
    pub obj_id: usize,
}

impl SceneIntersection {
    pub fn new(info: GeometryInfo, obj_id: usize) -> Self {
        Self { info, obj_id }
    }
}

pub struct Scene {
    objects: Store<SceneObject>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn push_obj(&mut self, obj: SceneObject) -> usize {
        self.objects.push(obj)
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_obj(&self, obj_id: usize) -> &SceneObject {
        &self.objects[obj_id]
    }

    pub fn is_occluded(&self, ray: &Ray) -> bool {
        self.objects
            .iter()
            .any(|object| object.intersect(ray).is_some())
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let mut ray = *ray;
        let mut index = None;

        for (i, obj) in self.objects.iter().enumerate() {
            if let Some(t) = obj.intersect(&ray) {
                index = Some(i);
                ray.t = t;
            }
        }

        if let Some(index) = index {
            let hit = Hit::from(ray);
            let info = self.objects[index].get_info(hit);

            Some(SceneIntersection::new(info, index))
        } else {
            None
        }
    }

    pub fn reflect_from(&self, intersection: SceneIntersection) -> Option<SceneIntersection> {
        let normal = intersection.info.normal;
        let mut direction = intersection.info.ray.direction;
        direction.reflect(normal);
        let ray = intersection.info.create_ray(direction);

        self.intersect(&ray)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Store::default(),
            lights: Vec::default(),
        }
    }
}
