use crate::color::Color;
use crate::geometry::ray::Ray;
use crate::render::bxdf::BxDFType;
use crate::render::integrator::Integrator;
use crate::render::sampler::Sampler;
use crate::render::scene::{Scene, SceneIntersection};
use crate::{floats, Spectrum};

pub struct Whitted;

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn illumination(
        &self,
        scene: &Scene,
        intersection: &SceneIntersection,
        sampler: &mut dyn Sampler,
    ) -> Spectrum {
        let bsdf = &intersection.obj.bsdf;

        let outgoing = &-intersection.info.ray.direction;
        let point = &intersection.info.point;
        let normal = &intersection.info.normal;

        let mut color = Spectrum::black();

        for light in &scene.lights {
            let mut ray = Ray::in_range(point, &light.position());
            ray.t_start += floats::EPSILON;

            let c = bsdf.evaluate(normal, &ray.direction, outgoing, BxDFType::ALL);

            if !c.is_black() {
                color += c;

                if !scene.is_occluded(&ray) {
                    let cos = ray.direction.dot(*normal).abs();
                    // let pdf = bsdf.pdf(normal, &ray.direction, outgoing, BxDFType::ALL);

                    color += c * cos;
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
