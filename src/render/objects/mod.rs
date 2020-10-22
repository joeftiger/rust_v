pub mod sphere_object;

use crate::geometry::ray::Ray;
use crate::geometry::{Boxable, Intersectable};
use crate::physics::rgb::SRGB;

pub trait SceneObject: Boxable + Intersectable<Ray> + Send + Sync {
    fn material(&self) -> SRGB;
}
