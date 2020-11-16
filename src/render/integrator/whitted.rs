use crate::render::bxdf::BxDFType;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;
use color::Color;

pub struct Whitted {
    pub max_depth: usize,
}

impl Whitted {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
}

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn integrate(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        self.illumination(scene, intersection, sampler, self.max_depth)
    }

    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
        depth: usize,
    ) -> Spectrum {
        let outgoing = &-intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut color = Spectrum::black();

        for light in &scene.lights {
            let light_sample = light.sample(intersection);
            let c = bsdf.evaluate(normal, &light_sample.incident, outgoing, BxDFType::ALL);

            if light_sample.pdf > 0.0
                && !light_sample.spectrum.is_black()
                && !c.is_black()
                && light_sample.occlusion_tester.unoccluded(scene)
            {
                let cos = light_sample.incident.dot(*normal).abs();

                if cos != 0.0 {
                    color += light_sample.spectrum * c * (cos / light_sample.pdf);
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
