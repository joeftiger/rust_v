use crate::render::bxdf::BxDFType;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use color::Color;
use geometry::ray::Ray;
use std::sync::Arc;

pub mod debug_normals;
pub mod path;
pub mod whitted;

pub trait Integrator: Send + Sync {
    fn integrate(&self, scene: &Scene, primary_ray: &Ray, sampler: Arc<dyn Sampler>) -> Spectrum;

    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: Arc<dyn Sampler>,
        depth: u32,
    ) -> Spectrum;

    //noinspection DuplicatedCode
    fn specular_reflection(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: Arc<dyn Sampler>,
        depth: u32,
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
                        let illumination = self.illumination(scene, &i, sampler, depth - 1);

                        return bxdf_sample.spectrum * illumination * cos / bxdf_sample.pdf;
                    }
                }
            }
        }

        Spectrum::black()
    }

    //noinspection DuplicatedCode
    fn specular_transmission(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: Arc<dyn Sampler>,
        depth: u32,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let normal = intersection.info.normal;
        let sample = sampler.get_sample();

        let bxdf_sample = bsdf.sample(
            &normal,
            &outgoing,
            BxDFType::SPECULAR | BxDFType::TRANSMISSION,
            &sample,
        );

        if let Some(bxdf_sample) = bxdf_sample {
            if bxdf_sample.pdf > 0.0 && !bxdf_sample.spectrum.is_black() {
                let cos = bxdf_sample.incident.dot(normal).abs();

                if cos != 0.0 {
                    let transmitted_ray = intersection.info.create_ray(bxdf_sample.incident);

                    if let Some(i) = scene.intersect(&transmitted_ray) {
                        let illumination = self.illumination(scene, &i, sampler, depth - 1);

                        return bxdf_sample.spectrum * illumination * cos / bxdf_sample.pdf;
                    }
                }
            }
        }

        Spectrum::black()
    }
}
