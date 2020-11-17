use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use ultraviolet::Vec3;

pub struct DebugNormals;

impl Integrator for DebugNormals {
    fn integrate(
        &self,
        _scene: &Scene,
        intersection: &SceneIntersection,
        _sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let color = (intersection.info.normal + Vec3::one()) / 2.0;

        color.into()
    }

    fn illumination(
        &self,
        _scene: &Scene,
        intersection: &SceneIntersection,
        _sampler: &mut dyn Sampler,
        _depth: i32,
    ) -> Spectrum {
        let color = (intersection.info.normal + Vec3::one()) / 2.0;

        color.into()
    }
}
