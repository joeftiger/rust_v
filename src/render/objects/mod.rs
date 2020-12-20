use crate::render::objects::emitter::Emitter;
use crate::render::objects::receiver::Receiver;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Boundable, Intersectable, Intersection};
use std::sync::Arc;

pub mod emitter;
pub mod receiver;
mod sphere;

#[derive(Clone)]
pub enum Instance {
    Emitter(Arc<dyn Emitter>),
    Receiver(Arc<dyn Receiver>),
}

impl Boundable for Instance {
    fn bounds(&self) -> Aabb {
        match self {
            Instance::Emitter(e) => e.bounds(),
            Instance::Receiver(r) => r.bounds(),
        }
    }
}

impl Intersectable for Instance {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Instance::Emitter(e) => e.intersect(ray),
            Instance::Receiver(r) => r.intersect(ray),
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        match self {
            Instance::Emitter(e) => e.intersects(ray),
            Instance::Receiver(r) => r.intersects(ray),
        }
    }
}
