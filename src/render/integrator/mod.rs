use crate::render::scene::{Scene, SceneIntersection};
use crate::Spectrum;

pub trait Integrator {
    fn illumination(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;

    fn specular_reflection(&self, _scene: &Scene, intersection: &SceneIntersection) -> Spectrum {
        let _outgoing = -intersection.info.ray.direction;

        unimplemented!()
    }

    fn specular_transmission(&self, scene: &Scene, intersection: &SceneIntersection) -> Spectrum;
}
