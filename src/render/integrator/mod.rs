pub mod debug_normals;
pub mod whitted;

use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use crate::render::bxdf::BxDFType;
use crate::color::Color;

pub trait Integrator {
    fn illumination(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;

    fn specular_reflection(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let normal = intersection.info.normal;
        let types = BxDFType::SPECULAR | BxDFType::REFLECTION;

        let sample = bsdf.sample(&normal, &outgoing, types, &sampler.get_2d());
        let cos = sample.incident.dot(normal).abs();

        let mut reflection = Spectrum::black();

        if sample.pdf > 0.0 && !sample.spectrum.is_black() && cos != 0.0 {
            let reflected_ray = intersection.info.create_ray(sample.incident);

            if let Some(i) = scene.intersect(&reflected_ray) {
                let illumination = self.illumination(scene, &i);

                reflection = sample.spectrum * illumination * cos / sample.pdf;
            }

        }

        reflection
    }

    fn specular_transmission(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;
}
