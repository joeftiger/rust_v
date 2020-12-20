use crate::bxdf::bsdf::BSDF;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Boundable, Geometry, Intersectable, Intersection};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Receiver: Geometry {
    fn shape(&self) -> &dyn Geometry;

    fn bsdf(&self) -> &Arc<BSDF>;
}

#[derive(Debug)]
pub struct ReceiverObj<T> {
    shape: T,
    bsdf: Arc<BSDF>,
}

impl<T> ReceiverObj<T> {
    pub fn new(shape: T, bsdf: Arc<BSDF>) -> Self {
        Self { shape, bsdf }
    }
}

impl<T> Boundable for ReceiverObj<T>
where
    T: Boundable,
{
    fn bounds(&self) -> Aabb {
        self.shape.bounds()
    }
}

impl<T> Intersectable for ReceiverObj<T>
where
    T: Intersectable,
{
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.shape.intersects(ray)
    }
}

impl<T> Receiver for ReceiverObj<T>
where
    T: Geometry,
{
    fn shape(&self) -> &dyn Geometry {
        &self.shape
    }

    fn bsdf(&self) -> &Arc<BSDF> {
        &self.bsdf
    }
}
