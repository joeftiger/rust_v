use crate::integrator::Integrator;
use crate::render::scene::{Scene, SceneIntersection};
use crate::sampler::Sampler;
use crate::Spectrum;
use color::Color;
use geometry::ray::Ray;
use std::sync::Arc;
use ultraviolet::Vec3;

pub struct DebugNormals;

impl Integrator for DebugNormals {
    #[inline(always)]
    fn integrate(&self, scene: &Scene, primary_ray: &Ray, _: Arc<dyn Sampler>) -> Spectrum {
        if let Some(si) = scene.intersect(primary_ray) {
            let color = (si.info.normal + Vec3::one()) / 2.0;

            color.into()
        } else {
            Spectrum::black()
        }
    }

    fn illumination(
        &self,
        _: &Scene,
        _: &SceneIntersection,
        _: Arc<dyn Sampler>,
        _: u32,
    ) -> Spectrum {
        unreachable!()
    }
}
