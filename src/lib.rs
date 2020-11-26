// To allow debug checks
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate bitflags;

use color::*;

pub mod cornell_box;
pub mod plain_scene;
pub mod render;
pub mod bunny;

pub type Spectrum = Srgb;
