pub mod debug_normals;

use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;

pub trait Integrator {
    fn illumination(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;

    #[allow(unused_variables)]
    fn specular_reflection(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let sample = sampler.get_2d();
        let bsdf = &intersection.obj.bsdf;

        unimplemented!()
    }

    fn specular_transmission(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;
}
