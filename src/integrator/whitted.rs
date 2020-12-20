use crate::bxdf::BxDFType;
use crate::integrator::Integrator;
use crate::render::objects::Instance;
use crate::render::scene::{Scene, SceneIntersection};
use crate::sampler::Sampler;
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

        let obj = match &intersection.obj {
            Instance::Emitter(e) => e.as_receiver(),
            Instance::Receiver(r) => r.clone(),
        };

        let bsdf = obj.bsdf();
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut illumination = Spectrum::black();

        for light in &scene.lights {
            // for _ in 0..LIGHT_SAMPLES_1D.min(scene.lights.len()) {
            //     let index = (sampler.get_1d() * scene.lights.len() as f32) as usize;
            //     let light = &scene.lights[index];
            let emitter_sample = light.sample(&intersection, &sampler.get_2d());

            if emitter_sample.pdf > 0.0
                && !emitter_sample.radiance.is_black()
                && !emitter_sample.occlusion_tester.is_occluded(scene)
            {
                let c = bsdf.evaluate(normal, &emitter_sample.incident, &outgoing, BxDFType::ALL);

                if !c.is_black() {
                    let cos = emitter_sample.incident.dot(*normal).abs();

                    if cos != 0.0 {
                        illumination += light.emission()
                            * c
                            * emitter_sample.radiance
                            * (cos / emitter_sample.pdf);
                    }
                }
            }
        }

        let new_depth = depth - 1;

        if new_depth > 0 {
            illumination +=
                self.specular_reflection(scene, intersection, sampler.clone(), new_depth);
            illumination += self.specular_transmission(scene, intersection, sampler, new_depth);
        }

        illumination
    }
}
