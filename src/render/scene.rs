use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Container, Geometry, GeometryInfo};
use std::rc::Rc;
use ultraviolet::Vec3;

/// Consists of
/// - info: [GeometryInfo](../geometry_leg/struct.GeometryInfo.html)
pub struct SceneIntersection {
    pub info: GeometryInfo,
    pub obj: Rc<SceneObject>,
}

impl SceneIntersection {
    pub fn new(info: GeometryInfo, obj: Rc<SceneObject>) -> Self {
        Self { info, obj }
    }
}

pub struct Scene {
    pub aabb: Aabb,
    pub lights: Vec<Rc<dyn Light>>,
    pub objects: Vec<Rc<SceneObject>>,
}

impl Scene {
    pub fn push_obj(&mut self, obj: SceneObject) {
        let aabb = &mut self.aabb;
        let bo = obj.bounding_box();
        aabb.min = aabb.min.min_by_component(bo.min);
        aabb.max = aabb.max.min_by_component(bo.min);

        self.objects.push(Rc::new(obj));
    }

    pub fn push_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(Rc::from(light));
    }

    /// Checks if the given ray intersects any object before reaching it's own maximum t lifespan.
    pub fn is_occluded(&self, ray: &Ray) -> bool {
        self.objects
            .iter()
            .any(|object| object.intersects(ray))
        //.any(|object| object.bounding_box().intersect(ray).is_some() && object.intersect(ray).is_some())
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let mut ray = *ray;
        let mut obj = None;
        let mut info: Option<GeometryInfo> = None;

        self.objects.iter().for_each(|so| {
            let new_info_op = so.intersect(&ray);
            if let Some(new_info) = new_info_op {
                obj = Some(so);
                info = new_info_op;
                ray.t_end = new_info.t;
            }
        });

        if let Some(obj) = obj {
            let info = info.unwrap();
            Some(SceneIntersection::new(info, obj.clone()))
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
            objects: Vec::default(),
        }
    }
}

impl Container for Scene {
    fn contains(&self, obj: Vec3) -> bool {
        self.aabb.contains(obj)
    }
}
