use crate::render::camera::Camera;
use crate::render::scene::Scene;

pub mod cornell_box;
pub mod spheres;

pub use cornell_box::CornellBox;
pub use spheres::Spheres;

pub const SIGMA: f32 = 20.0;
pub const FOVY: f32 = 70.0;

pub trait DemoScene {
    fn create(width: u32, height: u32) -> (Scene, Camera);
}
