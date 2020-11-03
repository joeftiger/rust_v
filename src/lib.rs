#[macro_use]
extern crate bitflags;

use crate::color::*;

pub mod color;
pub mod cornell_box;
pub mod floats;
pub mod geometry;
pub mod math;
pub mod render;
pub mod store;
pub mod util;

pub type Spectrum = Srgb;
