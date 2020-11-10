use crate::render::integrator::Integrator;
use crate::render::scene::{SceneIntersection, Scene};
use crate::Spectrum;

pub struct Whitted;

#[allow(unused_variables)]
impl Integrator for Whitted {
    fn illumination(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum {
        0.0.into()
    }

    fn specular_transmission(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum {
        0.0.into()
    }
}
