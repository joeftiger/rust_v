use crate::geometry::{Boxable, Intersectable};
use crate::geometry::ray::Ray;

pub mod material;

pub enum ColourMode {
    RGB,
    Spectral,
}

pub enum MonteCarlo {
    Random,
    Importance,
    HeroWaveLength,
}

pub trait SceneObject: Boxable + Intersectable<Box<dyn Ray>> {}

pub struct Scene {
    objects: Vec<Box<dyn SceneObject>>
}

pub trait Renderer {

}
