use crate::geometry::ray::Ray;
use crate::geometry::{Boxable, Intersectable};

pub trait SceneObject: Boxable + Intersectable<Ray> + Send + Sync {}
