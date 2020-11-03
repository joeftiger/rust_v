use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::{Container, Geometry, GeometryInfo};
use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use crate::store::Store;
use ultraviolet::Vec3;

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
    pub aabb: Aabb,
    pub lights: Vec<Light>,
    objects: Store<SceneObject>,
}

impl Scene {
    pub fn push_obj(&mut self, obj: SceneObject) -> usize {
        let aabb = &mut self.aabb;
        let bo = obj.bounding_box();
        aabb.min = aabb.min.min_by_component(bo.min);
        aabb.max = aabb.max.min_by_component(bo.min);

        self.objects.push(obj)
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_obj(&self, obj_id: usize) -> &SceneObject {
        &self.objects[obj_id]
    }

    /// Checks if the given ray intersects any object before reaching it's own maximum t lifespan.
    pub fn is_occluded(&self, ray: &Ray) -> bool {
        self.objects
            .iter()
            .any(|object| object.intersect(ray).is_some())
        //.any(|object| object.bounding_box().intersect(ray).is_some() && object.intersect(ray).is_some())
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let mut ray = *ray;
        let mut index = None;
        let mut info = None;

        for (i, obj) in self.objects.iter().enumerate() {
            // if obj.bounding_box().intersect(&ray).is_some() {
            if let Some(g_info) = obj.intersect(&ray) {
                index = Some(i);
                info = Some(g_info);
                ray.t = g_info.t;
            }
            // }
        }

        if let Some(index) = index {
            let info = info.expect("Unexpected erro");
            Some(SceneIntersection::new(info, index))
        } else {
            None
        }
    }

    pub fn reflect_from(&self, intersection: SceneIntersection) -> Option<SceneIntersection> {
        let direction = intersection
            .info
            .ray
            .direction
            .reflected(intersection.info.normal);
        let ray = intersection.info.create_ray(direction);

        self.intersect(&ray)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            aabb: Aabb::inverted_infinite(),
            lights: Vec::default(),
            objects: Store::default(),
        }
    }
}

impl Container for Scene {
    fn contains(&self, obj: Vec3) -> bool {
        self.aabb.contains(obj)
    }
}
