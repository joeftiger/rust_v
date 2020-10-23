use crate::color::Srgb;
use crate::geometry::aabb::Aabb;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::Geometry;
use crate::render::scene_objects::SceneObject;

pub struct SceneIntersection {
    pub intersection: Intersection,
    pub color: Srgb,
}

pub struct Scene<T> {
    objects: Vec<T>,
}

impl<T> Scene<T> {
    pub fn new(objects: Vec<T>) -> Self {
        Self { objects }
    }
}

impl<T: Geometry<Ray, Intersection>> Geometry<Ray, SceneIntersection> for Scene<SceneObject<T>> {
    fn bounding_box(&self) -> Aabb {
        let mut aabb = Aabb::inverted_infinite();
        self.objects
            .iter()
            .map(|o| o.bounding_box())
            .for_each(|bb| aabb = aabb.outer_join(&bb));

        aabb
    }

    fn intersect(&self, ray: &Ray) -> Option<SceneIntersection> {
        let pair = self
            .objects
            .iter()
            .map(|o| (o, o.intersect(ray)))
            .filter(|i| i.1.is_some())
            .map(|i| (i.0, i.1.unwrap()))
            .min_by(|i0, i1| i0.1.cmp_or_equal(&i1.1));

        if let Some(pair) = pair {
            let color = pair.0.get_color();

            Some(SceneIntersection {
                intersection: pair.1,
                color,
            })
        } else {
            None
        }
    }
}
