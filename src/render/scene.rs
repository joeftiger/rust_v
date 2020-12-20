use crate::render::bvh::{SceneBvh, SceneGeometry};
use crate::render::objects::emitter::Emitter;
use crate::render::objects::Instance;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Intersectable, Intersection};
use std::sync::Arc;

#[derive(Clone)]
pub struct SceneIntersection {
    pub info: Intersection,
    pub obj: Instance,
}

impl SceneIntersection {
    pub fn new(info: Intersection, obj: Instance) -> Self {
        Self { info, obj }
    }
}

pub struct Scene {
    pub aabb: Aabb,
    pub lights: Vec<Arc<dyn Emitter>>,
    pub objects: Vec<Instance>,
    bvh: Arc<SceneBvh>,
}

impl Scene {
    pub fn add(&mut self, obj: Instance) -> &mut Self {
        match &obj {
            Instance::Emitter(e) => {
                self.objects.push(obj.clone());
                self.lights.push(e.clone());
            }
            Instance::Receiver(_) => self.objects.push(obj.clone()),
        }

        self
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
