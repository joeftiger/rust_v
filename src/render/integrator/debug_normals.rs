use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use ultraviolet::Vec3;

pub struct DebugNormals;

impl Integrator for DebugNormals {
    fn illumination(
        &self,
        _scene: &Scene,
        intersection: &SceneIntersection,
        _sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let color = (intersection.info.normal + Vec3::one()) / 2.0;

        color.into()
    }

    fn specular_reflection(
        &self,
        _scene: &Scene,
        _intersection: &SceneIntersection,
        _sampler: &mut dyn Sampler,
    ) -> Spectrum {
        0.0.into()
    }

    fn specular_transmission(
        &self,
        _scene: &Scene,
        _intersection: &SceneIntersection,
        _sampler: &mut dyn Sampler,
    ) -> Spectrum {
        0.0.into()
    }
}
