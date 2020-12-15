pub mod bvh;
pub mod bxdf;
pub mod camera;
#[cfg(feature = "live-window")]
pub mod fast_window;
pub mod integrator;
pub mod light;
pub mod material;
pub mod renderer;
pub mod sampler;
pub mod scene;
pub mod scene_objects;

pub trait Named {
    fn name(&self) -> String;
}