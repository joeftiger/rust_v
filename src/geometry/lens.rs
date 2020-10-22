use crate::geometry::sphere::Sphere;
use crate::geometry::{Container, Geometry};
use ultraviolet::{Vec3, Vec3x4, f32x4};
use crate::geometry::ray::Ray;
use crate::geometry::intersection::Intersection;
use crate::geometry::aabb::Aabb;

pub struct BiconvexLens {
    pub sphere0: Sphere,
    pub sphere1: Sphere,
}

impl BiconvexLens {
    pub fn new(sphere0: Sphere, sphere1: Sphere) -> Self {
        debug_assert!((sphere0.center - sphere1.center).mag() < sphere0.radius.max(sphere1.radius));

        Self { sphere0, sphere1 }
    }
}

impl Container<Vec3, bool> for BiconvexLens {
    fn contains(&self, obj: Vec3) -> bool {
        self.sphere0.contains(obj) && self.sphere1.contains(obj)
    }
}

impl Container<Vec3x4, f32x4> for BiconvexLens {
    fn contains(&self, obj: Vec3x4) -> f32x4 {
        self.sphere0.contains(obj) & self.sphere1.contains(obj)
    }
}

impl Geometry<Ray, Intersection> for BiconvexLens {
    fn bounding_box(&self) -> Aabb {
        // not tight fitting, but okay enough
        let max = self.sphere0.radius.max(self.sphere1.radius);
        let offset = Vec3::one() * max;
        let center = (self.sphere0.center + self.sphere1.center) / 2.0;

        Aabb::new(center - offset, center + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(i0) = self.sphere0.intersect(ray) {
            if let Some(i1) = self.sphere1.intersect(ray) {
                // note the inversion of the first intersecting sphere
                return if i0.t < i1.t {
                    Some(i1)
                } else {
                    Some(i0)
                }
            }
        }

        None
    }
}

impl Default for BiconvexLens {
    fn default() -> Self {
        let offset = Vec3::unit_x() * 0.1;
        let sphere0 = Sphere::new(-offset, 1.0);
        let sphere1 = Sphere::new(offset, 1.0);

        BiconvexLens::new(sphere0, sphere1)
    }
}
