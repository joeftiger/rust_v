use geometry::ray::Ray;
use util::floats;

use crate::render::objects::Instance;
use crate::render::scene::SceneIntersection;
use geometry::bvh::Bvh;
use geometry::{Boundable, Intersectable};

pub type SceneBvh = Bvh<Instance>;

pub trait SceneGeometry {
    fn intersect_detailed(&self, ray: &Ray) -> Option<SceneIntersection>;
}

impl SceneGeometry for SceneBvh {
    fn intersect_detailed(&self, ray: &Ray) -> Option<SceneIntersection> {
        if !self.bounds().intersects(ray) {
            return None;
        }

        let obj_intersection = self
            .objects
            .iter()
            .filter_map(|o| Some(SceneIntersection::new(o.intersect(ray)?, o.clone())))
            .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t));

        if let Some(obj) = obj_intersection {
            let mut new_ray = *ray;
            new_ray.t_end = obj.info.t;

            let child_intersection = self
                .children
                .iter()
                .filter_map(|c| c.intersect_detailed(ray))
                .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t));

            if let Some(child) = child_intersection {
                if child.info.t < obj.info.t {
                    return Some(child);
                }
            }

            Some(obj)
        } else {
            self.children
                .iter()
                .filter_map(|c| c.intersect_detailed(ray))
                .min_by(|a, b| floats::fast_cmp(a.info.t, b.info.t))
        }
    }
}
