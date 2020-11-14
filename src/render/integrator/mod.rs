use crate::color::Color;
use crate::render::bxdf::BxDFType;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;

pub mod debug_normals;
pub mod whitted;

pub trait Integrator {
    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum;

    fn specular_reflection(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let normal = intersection.info.normal;
        let sample = sampler.get_sample();

        let bxdf_sample = bsdf.sample(
            &normal,
            &outgoing,
            BxDFType::SPECULAR | BxDFType::REFLECTION,
            &sample,
        );

        if let Some(bxdf_sample) = bxdf_sample {
            if bxdf_sample.pdf > 0.0 && !bxdf_sample.spectrum.is_black() {
                let cos = bxdf_sample.incident.dot(normal).abs();

                if cos != 0.0 {
                    let reflected_ray = intersection.info.create_ray(bxdf_sample.incident);

                    if let Some(i) = scene.intersect(&reflected_ray) {
                        let illumination = self.illumination(scene, &i, sampler);

                        return bxdf_sample.spectrum * illumination * cos / bxdf_sample.pdf;
                    }
                }
            }
        }

        Spectrum::black()
    }

    fn specular_transmission(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let normal = intersection.info.normal;
        let sample = sampler.get_sample();

        let bxdf_sample = bsdf.sample(
            &normal,
            &outgoing,
            BxDFType::SPECULAR | BxDFType::REFLECTION,
            &sample,
        );

        if let Some(bxdf_sample) = bxdf_sample {
            if bxdf_sample.pdf > 0.0 && !bxdf_sample.spectrum.is_black() {
                let cos = bxdf_sample.incident.dot(normal).abs();

                if cos != 0.0 {
                    let transmitted_ray = intersection.info.create_ray(bxdf_sample.incident);

                    if let Some(i) = scene.intersect(&transmitted_ray) {
                        let illumination = self.illumination(scene, &i, sampler);

                        return bxdf_sample.spectrum * illumination * cos / bxdf_sample.pdf;
                    }
                }
            }
        }

        Spectrum::black()
    }
}
