use crate::bxdf::BxDFType;
use crate::integrator::Integrator;
use crate::render::scene::{Scene, SceneIntersection};
use crate::sampler::Sampler;
use crate::Spectrum;
use color::Color;
use geometry::ray::Ray;
use std::sync::Arc;
use util::floats;

pub struct Path {
    pub min_depth: u32,
    pub max_depth: u32,
}

impl Path {
    pub fn new(min_depth: u32, max_depth: u32) -> Self {
        Self {
            min_depth,
            max_depth,
        }
    }
}

impl Integrator for Path {
    //noinspection DuplicatedCode
    fn integrate(&self, scene: &Scene, primary_ray: &Ray, sampler: Arc<dyn Sampler>) -> Spectrum {
        if let Some(si) = scene.intersect(primary_ray) {
            self.illumination(scene, &si, sampler, self.max_depth)
        } else {
            Spectrum::black()
        }
    }

    //noinspection DuplicatedCode
    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: Arc<dyn Sampler>,
        _: u32,
    ) -> Spectrum {
        let mut color = Spectrum::black();
        let mut throughput = Spectrum::new_const(1.0);

        let mut hit = intersection.clone();
        let mut specular = false;

        for bounce in 0..self.max_depth {
            let outgoing = -hit.info.ray.direction;

            let material = &hit.obj.material();
            let bsdf = &material.bsdf;
            let normal = &hit.info.normal;

            let mut illumination = Spectrum::black();

            if (bounce == 0 || specular) && material.emissive() {
                illumination += throughput * material.radiance(&outgoing, normal);
            }

            for light in &scene.lights {
                // for _ in 0..LIGHT_SAMPLES_1D.min(scene.lights.len()) {
                //     let index = (sampler.get_1d() * scene.lights.len() as f32) as usize;
                //     let light = &scene.lights[index];
                let light_tester = light.sample(&hit, &sampler.get_3d());

                if let Some(light_sample) = light_tester.test(scene) {
                    if light_sample.pdf > 0.0 {
                        let c =
                            bsdf.evaluate(normal, &light_sample.incident, &outgoing, BxDFType::ALL);

                        if !c.is_black() {
                            let cos = light_sample.incident.dot(*normal).abs();

                            if cos != 0.0 {
                                illumination += light.spectrum()
                                    * c
                                    * (light_sample.intensity * cos / light_sample.pdf);
                            }
                        }
                    }
                }
            }

            color += throughput * illumination;

            let sample = sampler.get_sample();

            let bxdf_sample = bsdf.sample(normal, &outgoing, BxDFType::ALL, &sample);
            if let Some(bxdf_sample) = bxdf_sample {
                if bxdf_sample.pdf == 0.0 || bxdf_sample.spectrum.is_black() {
                    break;
                }

                let dot = if bxdf_sample.typ.is_specular() {
                    specular = true;
                    1.0
                } else {
                    specular = false;
                    floats::fast_clamp(bxdf_sample.incident.dot(hit.info.normal).abs(), 0.0, 1.0)
                };

                throughput *= bxdf_sample.spectrum * (dot / bxdf_sample.pdf);

                // if bounce > self.min_depth {
                //     let const_prob = 0.75;
                //     if fastrand::f32() > const_prob {
                //         break;
                //     }
                //
                //     throughput /= const_prob;
                // }

                let ray = hit.info.create_ray(bxdf_sample.incident);

                match scene.intersect(&ray) {
                    Some(i) => hit = i,
                    None => return color,
                }
            } else {
                return color;
            }
        }

        color
    }
}
