use crate::geometry::{Boxable, Intersectable, Ray};
use serde::{Serialize, Deserialize};


pub trait SceneObject<'a>: Boxable + Intersectable<Ray> + Serialize + Deserialize<'a> {}

pub struct Scene {}
