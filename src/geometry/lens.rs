use crate::geometry::{Boxable, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::geometry::plane::Plane;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use ultraviolet::Vec3;

/// A geometrical simple lens consists of the intersection of two spheres.
pub struct SimpleLens {
    pub sphere0: Sphere,
    pub sphere1: Sphere,
}

impl SimpleLens {
    pub fn new(sphere0: Sphere, sphere1: Sphere) -> Self {
        Self { sphere0, sphere1 }
    }

    pub fn is_symmetric(&self) -> bool {
        f32::abs(self.sphere0.radius - self.sphere1.radius) <= f32::EPSILON
    }
}

impl Default for SimpleLens {
    fn default() -> Self {
        let offset = Vec3::new(0.1, 0.0, 0.0);
        Self::new(Sphere::new(-offset, 1.0), Sphere::new(offset, 1.0))
    }
}

impl Boxable for SimpleLens {
    fn bounding_box(&self) -> Option<Aabb> {
        let aabb0 = self.sphere0.bounding_box().unwrap();
        let aabb1 = self.sphere1.bounding_box().unwrap();
        let inner_join = aabb0.inner_join(&aabb1);

        Some(inner_join)
    }
}

impl Intersectable<Ray> for SimpleLens {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(i0) = self.sphere0.intersects(ray) {
            if let Some(i1) = self.sphere1.intersects(ray) {
                let lens_side: Intersection;

                // note the inversion of the first intersecting sphere
                if i0.t.unwrap() < i1.t.unwrap() {
                    lens_side = i1;
                } else {
                    lens_side = i0;
                }

                return Some(lens_side);
            }
        }

        None
    }
}

pub struct PlanoConvexLens {
    pub sphere: Sphere,
    pub plane: Plane,
}

impl PlanoConvexLens {
    pub fn new(sphere: Sphere, plane: Plane) -> Self {
        Self { sphere, plane }
    }
}

impl Boxable for PlanoConvexLens {
    fn bounding_box(&self) -> Option<Aabb> {
        self.sphere.bounding_box()
    }
}

impl Intersectable<Ray> for PlanoConvexLens {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(is) = self.sphere.intersects(ray) {}

        // TODO: IMPLEMENT
        unimplemented!();
    }
}
