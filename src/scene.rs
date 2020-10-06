use crate::geometry::{Boxable, Intersectable};
use serde::{Serialize, Deserialize};
use crate::geometry::ray::Ray;


pub trait SceneObject<'a, T: Ray>: Boxable + Intersectable<T> + Serialize + Deserialize<'a> {}

pub struct Scene {}
