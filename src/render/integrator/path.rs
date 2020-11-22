// use crate::render::integrator::Integrator;
// use crate::render::sampler::Sampler;
// use crate::render::scene::{Scene, SceneIntersection};
// use crate::Spectrum;
// use color::Color;
// use geometry::ray::Ray;
//
// pub struct Path {
//     pub max_depth: u32,
// }
//
// impl Path {
//     pub fn new(max_depth: u32) -> Self {
//         Self { max_depth }
//     }
// }
//
// impl Integrator for Path {
//     //noinspection DuplicatedCode
//     fn integrate(&self, scene: &Scene, primary_ray: &Ray, sampler: &mut dyn Sampler) -> Spectrum {
//         if let Some(si) = scene.intersect(primary_ray) {
//             self.illumination(scene, &si, sampler, self.max_depth)
//         } else {
//             Spectrum::black()
//         }
//     }
//
//     fn illumination(
//         &self,
//         scene: &Scene,
//         intersection: &SceneIntersection,
//         sampler: &mut dyn Sampler,
//         depth: u32,
//     ) -> Spectrum {
//         let num_samples = self.max_depth + 1;
//
//         let color = Spectrum::black();
//         unimplemented!()
//     }
// }
