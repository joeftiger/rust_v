use crate::color::Srgb;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use crate::render::scene_objects::SceneObject;
use crate::render::light::Light;
use crate::floats;

pub struct SceneIntersection {
    pub info: GeometryInfo,
    pub color: Srgb,
}

impl SceneIntersection {
    pub fn new(info: GeometryInfo, color: Srgb) -> Self {
        Self { info, color }
    }
}

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
}

/*
Is used as either
- SceneFilter<SceneObject, Option<f32> or
- SceneFilter<SceneObject, f32 or
- SceneFilter<Light, Option<f32> or
- SceneFilter<Light, f32
 */
#[derive(Copy, Clone)]
struct SceneFilter<TObject, TFloat> {
    pub obj: TObject,
    pub t: TFloat,
}

impl<TObject, TFloat> SceneFilter<TObject, TFloat> {
    fn new(obj: TObject, t: TFloat) -> Self {
        Self { obj, t }
    }
}

impl Scene {
    pub fn new(objects: Vec<SceneObject>, lights: Vec<Light>) -> Self {
        Self { objects, lights }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let object = self.objects
            .iter()
            .map(|object| SceneFilter::new(object, object.intersect(ray)))
            .filter(|filter| filter.t.is_some())
            .map(|filter| SceneFilter::new(filter.obj, filter.t.unwrap()))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        let light = self.lights
            .iter()
            .map(|object| SceneFilter::new(object, object.intersect(ray)))
            .filter(|filter| filter.t.is_some())
            .map(|filter| SceneFilter::new(filter.obj, filter.t.unwrap()))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        if let Some(object) = object {
            if let Some(light) = light {
                if light.t < object.t {
                    let hit = Hit::new(Ray::from(ray), light.t);
                    let info = light.obj.get_info(hit);

                    return Some(SceneIntersection::new(info, light.obj.color));
                }
            }

            let hit = Hit::new(Ray::from(ray), object.t);
            let info = object.obj.get_info(hit);

            Some(SceneIntersection::new(info, object.obj.color))
        } else if let Some(light) = light {
            let hit = Hit::new(Ray::from(ray), light.t);
            let info = light.obj.get_info(hit);

            Some(SceneIntersection::new(info, light.obj.color))
        } else {
            None
        }
    }
}
