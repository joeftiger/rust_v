use crate::geometry::{Boxable, Intersectable};
use crate::geometry::ray::Ray;

pub trait SceneObject: Boxable + Intersectable<Box<dyn Ray>> {}

