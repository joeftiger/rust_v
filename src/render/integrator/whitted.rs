use crate::render::bxdf::BxDFType;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use color::Color;
use geometry::ray::Ray;
use std::sync::Arc;

pub struct Whitted {
    pub max_depth: u32,
}

impl Whitted {
    pub fn new(max_depth: u32) -> Self {
        Self { max_depth }
    }
}

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn integrate(&self, scene: &Scene, primary_ray: &Ray, sampler: Arc<dyn Sampler>) -> Spectrum {
        if let Some(si) = scene.intersect(primary_ray) {
            self.illumination(scene, &si, sampler, self.max_depth)
        } else {
            Spectrum::black()
        }
    }

    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: Arc<dyn Sampler>,
        depth: u32,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.material.bsdf;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut illumination = Spectrum::black();

        for light in &scene.lights {
            let light_sample = light.sample(&intersection, &sampler.get_3d());

            if light_sample.pdf > 0.0 {
                if let Some((intensity, incident)) = light_sample.light_tester.test(scene) {
                    let c = bsdf.evaluate(normal, &incident, &outgoing, BxDFType::ALL);

                    if !c.is_black() {
                        let cos = incident.dot(*normal).abs();

                        if cos != 0.0 {
                            illumination += light.spectrum() * c * (intensity * cos / light_sample.pdf);
                        }
                    }
                }
            }
        }

        let new_depth = depth - 1;

        if new_depth > 0 {
            illumination += self.specular_reflection(scene, intersection, sampler.clone(), new_depth);
            illumination += self.specular_transmission(scene, intersection, sampler, new_depth);
        }

        illumination
    }
}
