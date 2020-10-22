use crate::geometry::sphere::Sphere;
use crate::physics::rgb::SRGB;
use crate::render::objects::SceneObject;
use crate::geometry::{Boxable, Intersectable, Intersection};
use crate::geometry::aabb::Aabb;
use crate::geometry::ray::Ray;

pub struct SphereObject {
    sphere: Sphere,
    material: SRGB,
}

impl SphereObject {
    pub fn new(sphere: Sphere, material: SRGB) -> Self {
        Self { sphere, material }
    }
}

impl SceneObject for SphereObject {
    fn material(&self) -> SRGB {
        self.material.clone()
    }
}

impl Boxable for SphereObject {
    fn bounding_box(&self) -> Option<Aabb> {
        self.sphere.bounding_box()
    }
}

impl Intersectable<Ray> for SphereObject {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        self.sphere.intersects(ray)
    }
}
