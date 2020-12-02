use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Container, Geometry, GeometryInfo};
use std::sync::Arc;
use ultraviolet::Vec3;
use crate::render::bvh::BvhNode;
use crate::render::bvh;

#[derive(Clone)]
pub struct SceneIntersection {
    pub info: GeometryInfo,
    pub obj: Arc<SceneObject>,
}

impl SceneIntersection {
    pub fn new(info: GeometryInfo, obj: Arc<SceneObject>) -> Self {
        Self { info, obj }
    }
}

pub struct Scene {
    pub aabb: Aabb,
    pub lights: Vec<Arc<dyn Light>>,
    pub objects: Vec<Arc<SceneObject>>,
    bvh: Arc<BvhNode>,
}

impl Scene {
    pub fn push_obj(&mut self, obj: SceneObject) {
        let aabb = &mut self.aabb;
        let bo = obj.bounding_box();
        aabb.min = aabb.min.min_by_component(bo.min);
        aabb.max = aabb.max.min_by_component(bo.min);

        self.objects.push(Arc::new(obj));
    }

    pub fn push_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn build_bvh(&mut self) {
        bvh::build_tree(self.objects.clone());
    }

    /// Checks if the given ray intersects any object before reaching it's own maximum t lifespan.
    pub fn is_occluded(&self, ray: &Ray) -> bool {
        self.bvh.intersects(ray)
        // self.objects.iter().any(|o| o.bounding_box().intersects(ray) && o.intersects(ray))
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        self.bvh.intersect(ray)
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
            bvh: Arc::new(BvhNode::new_empty()),
        }
    }
}

impl Container for Scene {
    fn contains(&self, obj: Vec3) -> bool {
        self.aabb.contains(obj)
    }
}
