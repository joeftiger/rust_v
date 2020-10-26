use crate::floats;
use crate::geometry::ray::Ray;
use crate::geometry::{Geometry, GeometryInfo, Hit};
use crate::render::light::Light;
use crate::render::scene_objects::SceneObject;
use std::ops::{Index, IndexMut};

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
    object_store: ObjectStore,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(objects: Vec<SceneObject>, lights: Vec<Light>) -> Self {
        Self { object_store: ObjectStore::new(objects), lights }
    }

    pub fn get_obj(&self, obj_id: usize) -> &SceneObject {
        &self.object_store[obj_id]
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let object = self
            .object_store
            .objects
            .iter()
            .map(|object| SceneFilter::new(object, object.intersect(ray)))
            .filter(|filter| filter.t.is_some())
            .map(|filter| SceneFilter::new(filter.obj, filter.t.unwrap()))
            .min_by(|a, b| floats::fast_cmp(a.t, b.t));

        let light = self
            .lights
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

                    return Some(SceneIntersection::new(info, light.obj.object.id));
                }
            }

            let hit = Hit::new(Ray::from(ray), object.t);
            let info = object.obj.get_info(hit);

            Some(SceneIntersection::new(info, object.obj.id))
        } else if let Some(light) = light {
            let hit = Hit::new(Ray::from(ray), light.t);
            let info = light.obj.get_info(hit);

            Some(SceneIntersection::new(info, light.obj.object.id))
        } else {
            None
        }
    }
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

struct ObjectStore {
    objects: Vec<SceneObject>
}

impl ObjectStore {
    pub fn new(objects: Vec<SceneObject>) -> Self {
        let mut store = Self { objects };

        for i in 0..store.objects.len() {
            store.objects[i].id = i;
        }

        store
    }
}

impl Index<usize> for ObjectStore {
    type Output = SceneObject;

    fn index(&self, index: usize) -> &Self::Output {
        &self.objects[index]
    }
}

impl IndexMut<usize> for ObjectStore {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.objects[index]
    }
}
