use crate::bxdf::bsdf::BSDF;
use crate::render::objects::receiver::{Receiver, ReceiverObj};
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use geometry::aabb::Aabb;
use geometry::ray::Ray;
use geometry::{Boundable, Geometry, Intersectable, Intersection};
use std::fmt::Debug;
use std::sync::Arc;
use ultraviolet::{Vec2, Vec3};
use util::floats;

pub trait Emitter: Receiver {
    fn as_receiver(&self) -> Arc<dyn Receiver + '_> {
        Arc::new(ReceiverObj::new(self.shape().clone(), self.bsdf().clone()))
    }

    fn emission(&self) -> Spectrum;

    #[inline]
    fn radiance(&self, incident: &Vec3, normal: &Vec3) -> Spectrum {
        let dot = incident.dot(*normal);

        if dot > 0.0 {
            self.emission()
        } else {
            Spectrum::new_const(0.0)
        }
    }

    fn sample(&self, intersection: &SceneIntersection, sample: &Vec2) -> EmitterSample;
}

#[derive(Debug)]
pub struct EmitterObj<T> {
    shape: T,
    bsdf: Arc<BSDF>,
    emission: Spectrum,
}

impl<T> EmitterObj<T> {
    pub fn new(shape: T, bsdf: Arc<BSDF>, emission: Spectrum) -> Self {
        Self {
            shape,
            bsdf,
            emission,
        }
    }
}

impl<T> Boundable for EmitterObj<T>
where
    T: Boundable,
{
    fn bounds(&self) -> Aabb {
        self.shape.bounds()
    }
}

impl<T> Intersectable for EmitterObj<T>
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

impl<T> Receiver for EmitterObj<T>
where
    T: Debug + Geometry,
{
    fn shape(&self) -> &dyn Geometry {
        &self.shape
    }

    fn bsdf(&self) -> &Arc<BSDF> {
        &self.bsdf
    }
}

impl<T> Emitter for EmitterObj<T>
where
    T: Debug + Geometry + Sampleable + Send + Sync,
{
    fn emission(&self) -> Spectrum {
        self.emission
    }

    fn sample(&self, intersection: &SceneIntersection, sample: &Vec2) -> EmitterSample {
        let point = intersection.info.point;
        let surface = self.shape.sample_surface(&point, sample);

        let incident = (surface.point - point).normalized();

        let from = point; // + intersection.info.normal * floats::EPSILON;
        let occlusion_tester = OcclusionTester::between(from, surface.point);

        let pdf = self.shape.pdf(&occlusion_tester.ray);
        let radiance = self.radiance(&incident, &surface.normal);

        EmitterSample::new(radiance, incident, pdf, occlusion_tester)
    }
}

pub struct EmitterSample {
    pub radiance: Spectrum,
    pub incident: Vec3,
    pub pdf: f32,
    pub occlusion_tester: OcclusionTester,
}

impl EmitterSample {
    pub fn new(
        radiance: Spectrum,
        incident: Vec3,
        pdf: f32,
        occlusion_tester: OcclusionTester,
    ) -> Self {
        Self {
            radiance,
            incident,
            pdf,
            occlusion_tester,
        }
    }
}

pub struct OcclusionTester {
    ray: Ray,
}

impl OcclusionTester {
    pub fn between(from: Vec3, to: Vec3) -> Self {
        let dir = to - from;
        let ray = Ray::with(
            from,
            dir.normalized(),
            floats::BIG_EPSILON,
            dir.mag() - floats::BIG_EPSILON,
        );

        Self { ray }
    }

    pub fn is_occluded(&self, scene: &Scene) -> bool {
        scene.is_occluded(&self.ray)
    }
}

pub struct SurfaceSample {
    pub point: Vec3,
    pub normal: Vec3,
}

impl SurfaceSample {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

/// A trait for objects (`Geometry` e.g.) that can sample a point on their surface.
pub trait Sampleable {
    /// The surface area of this object
    fn surface_area(&self) -> f32;

    /// Sample this object of the solid angle from `point` to the sampled point on the surface.
    fn sample_surface(&self, point: &Vec3, sample: &Vec2) -> SurfaceSample;

    /// Computes the PDF that the ray intersects this object.
    fn pdf(&self, ray: &Ray) -> f32;
}
