use crate::render::integrator::Integrator;
use crate::render::scene::{SceneIntersection, Scene};
use crate::{Spectrum, floats};
use crate::render::sampler::Sampler;
use crate::color::Color;
use crate::render::bxdf::BxDFType;

pub struct Whitted;

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn illumination(&self, scene: &Scene, intersection: &SceneIntersection, sampler: &mut dyn Sampler) -> Spectrum {
        let bsdf = &intersection.obj.bsdf;

        let outgoing = &-intersection.info.ray.direction;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut color = Spectrum::black();

        for light in &scene.lights {
            let mut ray = light.ray_from(point);
            ray.origin += *normal * floats::EPSILON;
            ray.t -= floats::BIG_EPSILON;

            let c = bsdf.evaluate(normal, &ray.direction, outgoing, BxDFType::ALL);

            if !c.is_black() {
                color += c;

                if !scene.is_occluded(&ray) {
                    let cos = ray.direction.dot(*normal).abs();
                    // let pdf = bsdf.pdf(normal, &ray.direction, outgoing, BxDFType::ALL);

                    color += c * light.intensity_at(point) * cos;
                }
            }
        }

        color
    }

    fn specular_transmission(&self, scene: &Scene, intersection: &SceneIntersection, sampler: &mut dyn Sampler) -> Spectrum {
        0.0.into()
    }
}
