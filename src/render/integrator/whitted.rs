use crate::render::bxdf::BxDFType;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use color::Color;
use geometry::ray::Ray;

pub struct Whitted {
    pub max_depth: i32,
}

impl Whitted {
    pub fn new(max_depth: i32) -> Self {
        Self { max_depth }
    }
}

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn integrate(&self, scene: &Scene, primary_ray: &Ray, sampler: &mut dyn Sampler) -> Spectrum {
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
        sampler: &mut dyn Sampler,
        depth: i32,
    ) -> Spectrum {
        let outgoing = -intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut color = Spectrum::black();

        for light in &scene.lights {
            let light_sample = light.sample(intersection);

            if light_sample.pdf > 0.0 && !light_sample.spectrum.is_black() {
                let c = bsdf.evaluate(normal, &light_sample.incident, &outgoing, BxDFType::ALL);

                if !c.is_black() && light_sample.occlusion_tester.unoccluded(scene) {
                    let cos = light_sample.incident.dot(*normal).abs();

                    if cos != 0.0 {
                        color += light_sample.spectrum * c * (cos / light_sample.pdf);
                    }
                }
            }
        }

        let new_depth = depth - 1;

        if new_depth > 0 {
            color += self.specular_reflection(scene, intersection, sampler, new_depth);
            color += self.specular_transmission(scene, intersection, sampler, new_depth);
        }

        color
    }
}
