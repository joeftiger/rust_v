// To allow debug checks
#![allow(clippy::float_cmp)]

#[macro_use]
extern crate bitflags;

use color::*;

pub mod cornell_box;
pub mod render;

pub type Spectrum = Srgb;
