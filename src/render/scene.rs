use crate::render::bvh::{SceneBvh, SceneGeometry};
use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Container, IntersectionInfo, Intersectable};
use std::sync::Arc;
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct SceneIntersection {
    pub info: IntersectionInfo,
    pub obj: Arc<SceneObject>,
}

impl SceneIntersection {
    pub fn new(info: IntersectionInfo, obj: Arc<SceneObject>) -> Self {
        Self { info, obj }
    }
}

pub struct Scene {
    pub aabb: Aabb,
    pub lights: Vec<Arc<dyn Light>>,
    pub objects: Vec<Arc<SceneObject>>,
    bvh: Arc<SceneBvh>,
}

impl Scene {
    pub fn push_obj(&mut self, obj: SceneObject) {
        let obj = Arc::new(obj);

        self.objects.push(obj.clone());
        // if obj.material.emissive() {
        //     self.push_light(obj)
        // }
    }

    pub fn push_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn build_bvh(&mut self) {
        self.bvh = SceneBvh::aac_vec(self.objects.clone());
    }

    /// Checks if the given ray intersects any object before reaching it's own maximum t lifespan.
    pub fn is_occluded(&self, ray: &Ray) -> bool {
        self.bvh.intersects(ray)
        // self.objects.iter().any(|o| o.bounding_box().intersects(ray) && o.intersects(ray))
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        self.bvh.intersect_detailed(ray)
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
            objects: Vec::default(),
            bvh: Arc::new(SceneBvh::default()),
        }
    }
}

impl Container for Scene {
    fn contains(&self, obj: &Vec3) -> bool {
        self.aabb.contains(obj)
    }
}
