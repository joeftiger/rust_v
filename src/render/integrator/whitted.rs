use crate::color::Color;
use crate::render::bxdf::BxDFType;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;

pub struct Whitted;

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let outgoing = &-intersection.info.ray.direction;

        let bsdf = &intersection.obj.bsdf;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut color = Spectrum::black();

        for light in &scene.lights {
            let light_sample = light.sample(intersection);
            let c = bsdf.evaluate(normal, &light_sample.incident, outgoing, BxDFType::ALL);

            if light_sample.pdf > 0.0 && !light_sample.spectrum.is_black() && !c.is_black() && light_sample.occlusion_tester.unoccluded(scene) {
                let cos = light_sample.incident.dot(*normal).abs();

                if cos != 0.0 {
                    color += light_sample.spectrum * c * (cos / light_sample.pdf);
                }
            }
        }

        color
    }

    fn specular_transmission(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        0.0.into()
    }
}
