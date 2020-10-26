use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::{Container, Geometry, GeometryInfo, Hit};
use ultraviolet::Vec3;

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

impl Container for BiconvexLens {
    fn contains(&self, obj: Vec3) -> bool {
        self.sphere0.contains(obj) && self.sphere1.contains(obj)
    }
}

impl Geometry for BiconvexLens {
    fn bounding_box(&self) -> Aabb {
        // not tight fitting, but okay enough
        let max = self.sphere0.radius.max(self.sphere1.radius);
        let offset = Vec3::one() * max;
        let center = (self.sphere0.center + self.sphere1.center) / 2.0;

        Aabb::new(center - offset, center + offset)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        if let Some(t0) = self.sphere0.intersect(ray) {
            if let Some(t1) = self.sphere1.intersect(ray) {
                // note the inversion of the first intersecting sphere
                if t0 < t1 {
                    return Some(t1);
                } else {
                    return Some(t0);
                }
            }
        }

        None
    }

    fn get_info(&self, _hit: Hit) -> GeometryInfo {
        unimplemented!();
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
